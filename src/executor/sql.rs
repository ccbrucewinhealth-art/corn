pub fn init_config() -> &'static str { "MSSQL" }

pub fn validate_input(sql: &str) -> Result<(), String> {
    if sql.trim().is_empty() { return Err("sql empty".to_string()); }
    Ok(())
}

pub fn execute_core(sql: &str) -> String {
    format!("sql simulated execute: {}", sql.trim())
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4601 } else { 5601 } }

pub fn to_response(msg: &str) -> String { msg.to_string() }
