use crate::logging;
use std::thread;

pub mod common;
pub mod execute;
pub mod parser;
pub mod compat_supervisorctl;

/// Code-ID: SRC-045
/// 初始化 supervisor 模組預設設定（回傳 mode）。
pub fn init_config() -> String {
    std::env::var("cronSupervisorMode")
        .or_else(|_| std::env::var("CORN_SUPERVISOR_MODE"))
        .unwrap_or_else(|_| "embedded".to_string())
}

/// Code-ID: SRC-045
/// 驗證 supervisor mode。
pub fn validate_input(mode: &str) -> Result<(), String> {
    match mode {
        "embedded" | "compat" => Ok(()),
        _ => Err(format!("invalid supervisor mode: {mode}")),
    }
}

/// Code-ID: SRC-045
/// supervisor 背景初始化核心流程（svc 相關）。
/// 1) 載入並解析 supervisord 設定
/// 2) 做最小驗證
/// 3) 依 mode 執行 status（embedded/compat）
pub fn execute_core() -> Result<String, String> {
    logging::info(&logging::step(2, 1, "啟動 supervisor execute_core"));
    let mode = init_config();
    logging::debug(&format!("[SRC-045] supervisor execute_core mode={} validate_input start", mode));
    validate_input(&mode)?;

    let path = parser::init_config();
    logging::debug(&format!("[SRC-045] supervisor execute_core parser path={}", path));
    let cfg = parser::execute_core(&path).unwrap_or_else(|err| {
        logging::warn(&format!(
            "[SRC-045] supervisor execute_core parser failed path={} err={} fallback=env_defaults",
            path, err
        ));
        common::SupervisorConfig::from_env_defaults()
    });
    logging::debug(&format!(
        "[SRC-045] supervisor execute_core config loaded mode={} enabled={} conf_file={} program={} programs={}",
        cfg.mode,
        cfg.enabled,
        cfg.conf_file,
        cfg.program_name,
        cfg.program_entries.len()
    ));

    // 啟動階段：若啟用且設定檔定義 autostart program，依序執行 start
    let mut startup_outputs: Vec<String> = vec![];
    if cfg.enabled {
        let autostart_programs = cfg
            .program_entries
            .iter()
            .filter(|v| v.autostart)
            .collect::<Vec<_>>();
        logging::debug(&format!(
            "[SRC-045] supervisor execute_core startup autostart_count={}",
            autostart_programs.len()
        ));

        for entry in autostart_programs {
            let mut per_program_cfg = cfg.clone();
            per_program_cfg.program_name = entry.name.clone();
            logging::debug(&format!(
                "[SRC-045] supervisor execute_core startup start program={} command={}",
                entry.name, entry.command
            ));

            let program_name = entry.name.clone();
            let command = entry.command.clone();
            thread::spawn(move || {
                logging::debug(&format!(
                    "[SRC-045] supervisor execute_core startup background launching program={} command={}",
                    program_name, command
                ));
                match execute::execute_core(&per_program_cfg, common::SupervisorAction::Start) {
                    Ok(start_output) => logging::debug(&format!(
                        "[SRC-045] supervisor execute_core startup background started program={} output={}",
                        program_name, start_output
                    )),
                    Err(err) => logging::error(&format!(
                        "[SRC-045] supervisor execute_core startup background failed program={} err={}",
                        program_name, err
                    )),
                }
            });
            startup_outputs.push(format!("{}=>background", entry.name));
        }
    }

    let output = execute::execute_core(&cfg, common::SupervisorAction::Status)?;
    logging::debug(&format!(
        "[SRC-045] supervisor execute_core execute finished output={}",
        output
    ));

    let startup_summary = if startup_outputs.is_empty() {
        "none".to_string()
    } else {
        startup_outputs.join(";")
    };
    Ok(format!(
        "supervisor execute_core ok mode={} startup={} output={}",
        mode, startup_summary, output
    ))
}
