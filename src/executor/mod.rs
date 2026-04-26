mod shell;
mod sql;

use crate::logging;

#[derive(Debug, Clone)]
pub enum JobType { Shell, Sql, Python, Js }

pub fn init_config() -> Vec<JobType> {
    let kinds = vec![JobType::Shell, JobType::Sql, JobType::Python, JobType::Js];
    logging::debug(&format!("[SRC-006] executor init_config kinds={}", kinds.len()));
    kinds
}

pub fn validate_input(command: &str) -> Result<(), String> {
    if command.trim().is_empty() { return Err("command empty".to_string()); }
    Ok(())
}

pub fn execute_core(kind: JobType, command: &str) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-006] executor execute_core kind={} command_len={}",
        kind.as_str(),
        command.len()
    ));
    validate_input(command)?;
    match kind {
        JobType::Shell => shell::execute_core(command),
        JobType::Sql => Ok(sql::execute_core(command)),
        JobType::Python => Ok(format!("python exec simulated: {}", command)),
        JobType::Js => Ok(format!("js exec simulated: {}", command)),
    }
}

impl JobType {
    fn as_str(&self) -> &'static str {
        match self {
            JobType::Shell => "shell",
            JobType::Sql => "sql",
            JobType::Python => "python",
            JobType::Js => "js",
        }
    }
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4401 } else { 5401 } }

pub fn to_response(output: &str) -> String { output.to_string() }
