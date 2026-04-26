use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::logging;
use super::common::{SupervisorConfig, SupervisorProgramEntry};

/// Code-ID: SRC-045
/// 回傳 supervisor 設定檔路徑（可被 env 覆蓋）。
pub fn init_config() -> String {
    let path = std::env::var("cronSupervisorConfFile")
        .unwrap_or_else(|_| "./bin/deploy/conf/supervisord.conf".to_string())
    ;
    logging::debug(&format!("[SRC-045] supervisor parser init_config path={}", path));
    path
}

/// Code-ID: SRC-045
/// 驗證 parser 輸入。
pub fn validate_input(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("supervisor config path is empty".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-045
/// 解析 `supervisord.conf`（INI 子集合）：section + key/value。
pub fn execute_core(path: &str) -> Result<SupervisorConfig, String> {
    logging::debug(&format!(
        "[SRC-045] supervisor parser execute_core loading path={}",
        path
    ));
    validate_input(path)?;

    let mut cfg = SupervisorConfig::from_env_defaults();
    cfg.conf_file = path.to_string();

    if !Path::new(path).exists() {
        logging::debug("[SRC-045] supervisor parser config not found, fallback env defaults");
        return Ok(cfg);
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    logging::debug(&format!(
        "[SRC-045] supervisor parser config loaded bytes={}",
        content.len()
    ));
    cfg.sections = parse_ini_like(&content);

    // 支援 [include] files=...，將額外設定檔合併進 sections
    load_include_sections(path, &mut cfg.sections);

    cfg.program_entries = collect_program_entries(&cfg.sections);
    logging::debug(&format!(
        "[SRC-045] supervisor parser discovered programs={} names={}",
        cfg.program_entries.len(),
        cfg.program_entries
            .iter()
            .map(|v| v.name.as_str())
            .collect::<Vec<_>>()
            .join(",")
    ));

    if let Some(server) = cfg.sections.get("supervisorctl") {
        if let Some(v) = server.get("serverurl") {
            cfg.unix_socket = v.to_string();
        }
    }

    if let Some(supervisord) = cfg.sections.get("supervisord") {
        if let Some(v) = supervisord.get("enabled") {
            cfg.enabled = matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes");
            logging::debug(&format!(
                "[SRC-045] supervisor parser mapped [supervisord].enabled={} => cfg.enabled={}",
                v, cfg.enabled
            ));
        }
    }

    if let Some(program) = cfg.program_entries.first() {
        cfg.program_name = program.name.clone();
        logging::debug(&format!(
            "[SRC-045] supervisor parser selected default program={} autostart={} command={}",
            cfg.program_name, program.autostart, program.command
        ));
    }

    Ok(cfg)
}

pub fn map_error_code(err: &str) -> i32 {
    if err.contains("empty") {
        4160
    } else {
        5160
    }
}

pub fn to_response(cfg: &SupervisorConfig) -> String {
    format!(
        "mode={} conf={} program={} sections={}",
        cfg.mode,
        cfg.conf_file,
        cfg.program_name,
        cfg.sections.len()
    )
}

fn parse_ini_like(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut out = HashMap::<String, HashMap<String, String>>::new();
    let mut current = String::new();

    for raw in content.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current = line[1..line.len() - 1].trim().to_string();
            out.entry(current.clone()).or_default();
            continue;
        }

        if let Some((k, v)) = line.split_once('=') {
            let key = k.trim().to_string();
            let val = v.trim().to_string();
            out.entry(current.clone()).or_default().insert(key, val);
        }
    }

    out
}

fn collect_program_entries(
    sections: &HashMap<String, HashMap<String, String>>,
) -> Vec<SupervisorProgramEntry> {
    let mut out = vec![];
    for (section, kv) in sections {
        if !section.starts_with("program:") {
            continue;
        }

        let name = section.trim_start_matches("program:").trim().to_string();
        if name.is_empty() {
            continue;
        }

        let command = kv.get("command").cloned().unwrap_or_default();
        let autostart = kv
            .get("autostart")
            .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);

        out.push(SupervisorProgramEntry {
            name,
            command,
            autostart,
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn load_include_sections(
    main_conf_path: &str,
    sections: &mut HashMap<String, HashMap<String, String>>,
) {
    let include_files_expr = sections
        .get("include")
        .and_then(|v| v.get("files"))
        .cloned();

    let Some(expr) = include_files_expr else {
        return;
    };

    let base_dir = Path::new(main_conf_path)
        .parent()
        .unwrap_or_else(|| Path::new("."));

    for raw_pattern in expr.split_whitespace() {
        let pattern = raw_pattern.trim();
        if pattern.is_empty() {
            continue;
        }

        logging::debug(&format!(
            "[SRC-045] supervisor parser include pattern={}",
            pattern
        ));

        let include_paths = expand_include_pattern(base_dir, pattern);
        logging::debug(&format!(
            "[SRC-045] supervisor parser include expanded pattern={} files={}",
            pattern,
            include_paths.len()
        ));

        for include_path in include_paths {
            let include_path_str = include_path.to_string_lossy().to_string();
            if !include_path.exists() {
                logging::debug(&format!(
                    "[SRC-045] supervisor parser include skip missing path={}",
                    include_path_str
                ));
                continue;
            }

            logging::debug(&format!(
                "[SRC-045] supervisor parser include load path={}",
                include_path_str
            ));
            match fs::read_to_string(&include_path) {
                Ok(content) => {
                    let child = parse_ini_like(&content);
                    merge_sections(sections, child);
                }
                Err(err) => {
                    logging::warn(&format!(
                        "[SRC-045] supervisor parser include read failed path={} err={}",
                        include_path_str, err
                    ));
                }
            }
        }
    }
}

fn expand_include_pattern(base_dir: &Path, pattern: &str) -> Vec<PathBuf> {
    let path = Path::new(pattern);
    let normalized = if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    };

    let normalized_str = normalized.to_string_lossy();
    if !normalized_str.contains('*') {
        return vec![normalized];
    }

    let Some(star_idx) = normalized_str.rfind('*') else {
        return vec![normalized];
    };

    let last_slash = normalized_str[..star_idx].rfind('/').unwrap_or(0);
    let (dir_part, file_pattern) = if last_slash == 0 {
        ("/", &normalized_str[1..])
    } else {
        (&normalized_str[..last_slash], &normalized_str[last_slash + 1..])
    };

    let dir = Path::new(dir_part);
    let Ok(entries) = fs::read_dir(dir) else {
        return vec![];
    };

    let (prefix, suffix) = file_pattern
        .split_once('*')
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .unwrap_or_default();

    let mut out = vec![];
    for entry in entries.flatten() {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with(&prefix) && file_name.ends_with(&suffix) {
            out.push(entry.path());
        }
    }
    out.sort();
    out
}

fn merge_sections(
    target: &mut HashMap<String, HashMap<String, String>>,
    source: HashMap<String, HashMap<String, String>>,
) {
    for (section, kv) in source {
        target.entry(section).or_default().extend(kv);
    }
}
