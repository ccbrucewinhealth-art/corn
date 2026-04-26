use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use once_cell::sync::Lazy;
use crate::logging;
use super::common::{SupervisorAction, SupervisorConfig};

static EMBEDDED_CHILDREN: Lazy<Mutex<HashMap<String, Child>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Code-ID: SRC-045
/// 執行前準備（保留擴充點）。
pub fn init_config() -> String {
    "supervisor-executor-ready".to_string()
}

/// Code-ID: SRC-045
/// 執行前驗證。
pub fn validate_input(cfg: &SupervisorConfig, action: SupervisorAction) -> Result<(), String> {
    if cfg.mode != "embedded" && cfg.mode != "compat" {
        return Err(format!("unsupported supervisor mode={}", cfg.mode));
    }
    if cfg.program_name.trim().is_empty() {
        return Err("cronSupervisorProgramName is empty".to_string());
    }
    if action.as_str().is_empty() {
        return Err("action is empty".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-045
/// 根據 mode 執行 status/start/stop/restart。
pub fn execute_core(cfg: &SupervisorConfig, action: SupervisorAction) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-045] supervisor execute_core mode={} action={}",
        cfg.mode,
        action.as_str()
    ));
    validate_input(cfg, action)?;

    if !cfg.enabled {
        logging::debug("[SRC-045] supervisor execute_core skipped because cronSupervisorEnabled=false");
        return Ok("supervisor disabled by cronSupervisorEnabled=false".to_string());
    }

    match cfg.mode.as_str() {
        "embedded" => {
            logging::debug("[SRC-045] supervisor execute_core dispatch embedded");
            execute_embedded(cfg, action)
        }
        "compat" => {
            logging::debug("[SRC-045] supervisor execute_core dispatch compat");
            execute_compat(cfg, action)
        }
        _ => Err(format!("invalid mode={}", cfg.mode)),
    }
}

pub fn map_error_code(err: &str) -> i32 {
    if err.contains("unsupported") || err.contains("invalid") {
        4161
    } else {
        5161
    }
}

pub fn to_response(output: &str) -> String {
    output.to_string()
}

fn execute_embedded(cfg: &SupervisorConfig, action: SupervisorAction) -> Result<String, String> {
    let autostart_names = cfg
        .program_entries
        .iter()
        .filter(|v| v.autostart)
        .map(|v| v.name.as_str())
        .collect::<Vec<_>>()
        .join(",");

    logging::debug(&format!(
        "[SRC-045] supervisor embedded action={} programs={} autostart=[{}]",
        action.as_str(),
        cfg.program_entries.len(),
        autostart_names
    ));

    let command = resolve_program_command(cfg);
    logging::debug(&format!(
        "[SRC-045] supervisor embedded resolved command program={} command={}",
        cfg.program_name, command
    ));

    match action {
        SupervisorAction::Status => {
            logging::debug("[SRC-045] supervisor embedded step=status");
            embedded_status(cfg, &autostart_names)
        }
        SupervisorAction::Start => {
            logging::debug("[SRC-045] supervisor embedded step=start");
            embedded_start(cfg, &command, &autostart_names)
        }
        SupervisorAction::Stop => {
            logging::debug("[SRC-045] supervisor embedded step=stop");
            embedded_stop(cfg, &autostart_names)
        }
        SupervisorAction::Restart => {
            logging::debug("[SRC-045] supervisor embedded step=restart(stop->start)");
            let _ = embedded_stop(cfg, &autostart_names);
            embedded_start(cfg, &command, &autostart_names)
        }
    }
}

fn embedded_status(cfg: &SupervisorConfig, autostart_names: &str) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-045] supervisor embedded status begin program={} autostart=[{}]",
        cfg.program_name, autostart_names
    ));
    let mut children = EMBEDDED_CHILDREN
        .lock()
        .map_err(|_| "embedded process registry poisoned".to_string())?;

    let state = if let Some(child) = children.get_mut(&cfg.program_name) {
        match child.try_wait() {
            Ok(Some(status)) => {
                let code = status
                    .code()
                    .map(|v| v.to_string())
                    .unwrap_or_else(|| "signal".to_string());
                children.remove(&cfg.program_name);
                format!("EXITED({})", code)
            }
            Ok(None) => "RUNNING".to_string(),
            Err(err) => format!("ERROR({})", err),
        }
    } else {
        "STOPPED".to_string()
    };

    let pid = children
        .get(&cfg.program_name)
        .map(|c| c.id().to_string())
        .unwrap_or_else(|| "-".to_string());

    logging::debug(&format!(
        "[SRC-045] supervisor embedded status result program={} state={} pid={}",
        cfg.program_name, state, pid
    ));

    Ok(format!(
        "embedded status program={} state={} pid={} timeout={}s loaded_programs={} autostart=[{}]",
        cfg.program_name,
        state,
        pid,
        cfg.ctl_timeout_sec,
        cfg.program_entries.len(),
        autostart_names
    ))
}

fn embedded_start(cfg: &SupervisorConfig, command: &str, autostart_names: &str) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-045] supervisor embedded start begin program={} command={} autostart=[{}]",
        cfg.program_name, command, autostart_names
    ));
    if command.trim().is_empty() {
        return Err(format!(
            "embedded start program={} failed: empty command",
            cfg.program_name
        ));
    }

    let mut children = EMBEDDED_CHILDREN
        .lock()
        .map_err(|_| "embedded process registry poisoned".to_string())?;

    if let Some(existing) = children.get_mut(&cfg.program_name) {
        match existing.try_wait() {
            Ok(None) => {
                logging::debug(&format!(
                    "[SRC-045] supervisor embedded start skip already-running program={} pid={}",
                    cfg.program_name,
                    existing.id()
                ));
                return Ok(format!(
                    "embedded start program={} already-running pid={} loaded_programs={} autostart=[{}]",
                    cfg.program_name,
                    existing.id(),
                    cfg.program_entries.len(),
                    autostart_names
                ));
            }
            Ok(Some(_)) => {
                children.remove(&cfg.program_name);
            }
            Err(err) => {
                return Err(format!(
                    "embedded start program={} failed while checking current process err={}",
                    cfg.program_name, err
                ));
            }
        }
    }

    let runtime = resolve_program_runtime(cfg);
    prepare_runtime_paths(cfg, &runtime)?;

    let mut cmd = Command::new("/bin/bash");
    cmd.arg("-lc").arg(command);

    if let Some(directory) = runtime.directory.as_deref() {
        logging::debug(&format!(
            "[SRC-045] supervisor embedded start set current_dir program={} directory={}",
            cfg.program_name, directory
        ));
        cmd.current_dir(directory);
    }

    if let Some(stdout_logfile) = runtime.stdout_logfile.as_deref() {
        let file = File::options()
            .create(true)
            .append(true)
            .open(stdout_logfile)
            .map_err(|err| {
                format!(
                    "embedded start program={} open stdout_logfile failed path={} err={}",
                    cfg.program_name, stdout_logfile, err
                )
            })?;
        cmd.stdout(Stdio::from(file));
    } else {
        cmd.stdout(Stdio::inherit());
    }

    if let Some(stderr_logfile) = runtime.stderr_logfile.as_deref() {
        let file = File::options()
            .create(true)
            .append(true)
            .open(stderr_logfile)
            .map_err(|err| {
                format!(
                    "embedded start program={} open stderr_logfile failed path={} err={}",
                    cfg.program_name, stderr_logfile, err
                )
            })?;
        cmd.stderr(Stdio::from(file));
    } else {
        cmd.stderr(Stdio::inherit());
    }

    let child = cmd.spawn().map_err(|err| {
        format!(
            "embedded start program={} spawn failed err={} command={}",
            cfg.program_name, err, command
        )
    })?;

    let pid = child.id();
    logging::debug(&format!(
        "[SRC-045] supervisor embedded start spawned program={} pid={}",
        cfg.program_name, pid
    ));
    children.insert(cfg.program_name.clone(), child);

    Ok(format!(
        "embedded start program={} pid={} command={} loaded_programs={} autostart=[{}]",
        cfg.program_name,
        pid,
        command,
        cfg.program_entries.len(),
        autostart_names
    ))
}

fn embedded_stop(cfg: &SupervisorConfig, autostart_names: &str) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-045] supervisor embedded stop begin program={} autostart=[{}]",
        cfg.program_name, autostart_names
    ));
    let mut children = EMBEDDED_CHILDREN
        .lock()
        .map_err(|_| "embedded process registry poisoned".to_string())?;

    if let Some(mut child) = children.remove(&cfg.program_name) {
        let pid = child.id();
        logging::debug(&format!(
            "[SRC-045] supervisor embedded stop killing program={} pid={}",
            cfg.program_name, pid
        ));
        if let Err(err) = child.kill() {
            return Err(format!(
                "embedded stop program={} pid={} kill failed err={}",
                cfg.program_name, pid, err
            ));
        }
        let _ = child.wait();
        logging::debug(&format!(
            "[SRC-045] supervisor embedded stop done program={} pid={}",
            cfg.program_name, pid
        ));
        return Ok(format!(
            "embedded stop program={} pid={} loaded_programs={} autostart=[{}]",
            cfg.program_name,
            pid,
            cfg.program_entries.len(),
            autostart_names
        ));
    }

    Ok(format!(
        "embedded stop program={} already-stopped loaded_programs={} autostart=[{}]",
        cfg.program_name,
        cfg.program_entries.len(),
        autostart_names
    ))
}

fn resolve_program_command(cfg: &SupervisorConfig) -> String {
    logging::debug(&format!(
        "[SRC-045] supervisor resolve_program_command begin program={} entries={}",
        cfg.program_name,
        cfg.program_entries.len()
    ));
    cfg.program_entries
        .iter()
        .find(|v| v.name == cfg.program_name)
        .map(|v| v.command.clone())
        .or_else(|| cfg.program_entries.first().map(|v| v.command.clone()))
        .unwrap_or_default()
}

#[derive(Debug, Default)]
struct ProgramRuntime {
    directory: Option<String>,
    stdout_logfile: Option<String>,
    stderr_logfile: Option<String>,
}

fn resolve_program_runtime(cfg: &SupervisorConfig) -> ProgramRuntime {
    let key = format!("program:{}", cfg.program_name);
    let Some(section) = cfg.sections.get(&key) else {
        logging::debug(&format!(
            "[SRC-045] supervisor resolve_program_runtime missing section={} use defaults",
            key
        ));
        return ProgramRuntime::default();
    };

    let runtime = ProgramRuntime {
        directory: section
            .get("directory")
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty()),
        stdout_logfile: section
            .get("stdout_logfile")
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty()),
        stderr_logfile: section
            .get("stderr_logfile")
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty()),
    };

    logging::debug(&format!(
        "[SRC-045] supervisor resolve_program_runtime program={} directory={:?} stdout_logfile={:?} stderr_logfile={:?}",
        cfg.program_name,
        runtime.directory,
        runtime.stdout_logfile,
        runtime.stderr_logfile
    ));
    runtime
}

fn prepare_runtime_paths(cfg: &SupervisorConfig, runtime: &ProgramRuntime) -> Result<(), String> {
    for path in [
        runtime.stdout_logfile.as_deref(),
        runtime.stderr_logfile.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        let parent = Path::new(path)
            .parent()
            .map(|v| v.to_path_buf())
            .unwrap_or_default();
        if parent.as_os_str().is_empty() {
            continue;
        }

        if !parent.exists() {
            logging::debug(&format!(
                "[SRC-045] supervisor embedded start create log dir program={} path={}",
                cfg.program_name,
                parent.to_string_lossy()
            ));
            fs::create_dir_all(&parent).map_err(|err| {
                format!(
                    "embedded start program={} create log dir failed path={} err={}",
                    cfg.program_name,
                    parent.to_string_lossy(),
                    err
                )
            })?;
        }
    }

    if let Some(directory) = runtime.directory.as_deref() {
        let workdir = Path::new(directory);
        if !workdir.exists() {
            logging::debug(&format!(
                "[SRC-045] supervisor embedded start create working dir program={} path={}",
                cfg.program_name, directory
            ));
            fs::create_dir_all(workdir).map_err(|err| {
                format!(
                    "embedded start program={} create directory failed path={} err={}",
                    cfg.program_name, directory, err
                )
            })?;
        }
    }

    Ok(())
}

fn execute_compat(cfg: &SupervisorConfig, action: SupervisorAction) -> Result<String, String> {
    let cmd_action = action.as_str();
    logging::debug(&format!(
        "[SRC-045] supervisor compat invoke bin={} conf={} action={} program={}",
        cfg.ctl_path, cfg.conf_file, cmd_action, cfg.program_name
    ));
    let output = Command::new(&cfg.ctl_path)
        .arg("-c")
        .arg(&cfg.conf_file)
        .arg(cmd_action)
        .arg(&cfg.program_name)
        .output();

    match output {
        Ok(v) if v.status.success() => {
            logging::debug(&format!(
                "[SRC-045] supervisor compat success action={} status={}",
                cmd_action, v.status
            ));
            Ok(String::from_utf8_lossy(&v.stdout).trim().to_string())
        }
        Ok(v) => Err(format!(
            "compat action={} failed: {}",
            cmd_action,
            String::from_utf8_lossy(&v.stderr).trim()
        )),
        Err(err) => {
            logging::warn(&format!(
                "[SRC-045] supervisor compat command not available fallback simulated action={} program={} err={}",
                cmd_action, cfg.program_name, err
            ));
            Ok(format!(
                "compat simulated action={} program={} reason={}",
                cmd_action, cfg.program_name, err
            ))
        }
    }
}
