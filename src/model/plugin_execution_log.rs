#[derive(Debug, Clone)]
pub struct PluginExecutionLog {
    pub plugin_id: String,
    pub version: String,
    pub status: String,
    pub duration_ms: i32,
}

pub fn init_config() -> PluginExecutionLog {
    PluginExecutionLog { plugin_id: "demo".into(), version: "1.0.0".into(), status: "OK".into(), duration_ms: 10 }
}

pub fn validate_input(v: &PluginExecutionLog) -> Result<(), String> {
    if v.plugin_id.is_empty() { return Err("plugin_id empty".into()); }
    Ok(())
}

pub fn execute_core(v: PluginExecutionLog) -> Result<PluginExecutionLog, String> {
    validate_input(&v)?;
    Ok(v)
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4801 } else { 5801 } }

pub fn to_response(v: &PluginExecutionLog) -> String { format!("{}:{}:{}", v.plugin_id, v.version, v.status) }
