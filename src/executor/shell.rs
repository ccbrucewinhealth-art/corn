use std::process::{Command, Stdio};

use crate::logging;

pub fn init_config() -> String { "bash".to_string() }

pub fn validate_input(command: &str) -> Result<(), String> {
    if command.trim().is_empty() { return Err("shell command empty".to_string()); }
    Ok(())
}

pub fn execute_core(command: &str) -> Result<String, String> {
    validate_input(command)?;

    logging::info(&format!("Exec {}", command));

    let output = Command::new("bash")
        .arg("-lc")
        .arg(command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("exit={}", output.status))
    } else {
        Err(format!("command failed status={}", output.status))
    }
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4501 } else { 5501 } }

pub fn to_response(result: &Result<String, String>) -> String {
    match result { Ok(v) => format!("ok:{}", v.trim()), Err(e) => format!("err:{}", e.trim()) }
}
