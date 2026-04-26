pub fn init_config() -> &'static str {
    "CornJobs"
}

pub fn validate_input(table: &str) -> Result<(), String> {
    if table.trim().is_empty() { return Err("table empty".to_string()); }
    Ok(())
}

pub fn execute_core(table: &str) -> String {
    format!(
        "CREATE TABLE IF NOT EXISTS {} (JobId TEXT PRIMARY KEY, CronExpr TEXT, Enabled INTEGER);",
        table
    )
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4301 } else { 5301 } }

pub fn to_response(sql: &str) -> String { sql.to_string() }
