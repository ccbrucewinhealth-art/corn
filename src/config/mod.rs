use std::collections::HashMap;

use crate::{cron, executor, proxy_mod, supervisor_mod};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub env: String,
    pub host: String,
    pub port: u16,
    pub extra: HashMap<String, String>,
}

pub fn init_config() -> AppConfig {
    let app_name = std::env::var("CORN_APP_NAME").unwrap_or_else(|_| "corn".to_string());
    let env = std::env::var("CORN_APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let host = std::env::var("CORN_APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("CORN_APP_PORT").ok().and_then(|v| v.parse().ok()).unwrap_or(8080);

    AppConfig { app_name, env, host, port, extra: HashMap::new() }
}

pub fn validate_input(cfg: &AppConfig) -> Result<(), String> {
    if cfg.app_name.trim().is_empty() {
        return Err("CORN_APP_NAME is empty".to_string());
    }
    if cfg.port == 0 {
        return Err("CORN_APP_PORT invalid".to_string());
    }
    Ok(())
}

pub fn execute_core() -> Result<AppConfig, String> {
    let cfg = init_config();
    validate_input(&cfg)?;
    Ok(cfg)
}

/// Code-ID: SRC-002
/// 服務模式（svc）相關子模組初始化整合：
/// - cron::mode init_config
/// - executor::mode init_config
/// - supervisor::mod init_config
pub fn init_config_svc_rel() -> Result<String, String> {
    crate::logging::debug("[SRC-002] init_config_svc_rel start");
    let cron_cfg = cron::init_config();
    crate::logging::debug(&format!("[SRC-002] cron init_config loaded file={}", cron_cfg));

    let executor_cfg = executor::init_config();
    crate::logging::debug(&format!(
        "[SRC-002] executor init_config loaded kinds={}",
        executor_cfg.len()
    ));

    let supervisor_cfg = supervisor_mod::init_config();
    crate::logging::debug(&format!(
        "[SRC-002] supervisor init_config loaded mode={}",
        supervisor_cfg
    ));

    let proxy_cfg_path = proxy_mod::parser::init_config();
    crate::logging::debug(&format!(
        "[SRC-002] proxy parser init_config loaded path={}",
        proxy_cfg_path
    ));

    supervisor_mod::validate_input(&supervisor_cfg)?;
    proxy_mod::parser::validate_input(&proxy_cfg_path)?;

    Ok(format!(
        "svc-rel initialized: cron={}, executor_kinds={}, supervisor_mode={}, proxy_cfg={}",
        cron_cfg,
        executor_cfg.len(),
        supervisor_cfg,
        proxy_cfg_path
    ))
}

pub fn map_error_code(err: &str) -> i32 {
    if err.contains("empty") { 4001 } else { 5001 }
}

pub fn to_response(cfg: &AppConfig) -> String {
    format!("name={}, env={}, bind={}:{}", cfg.app_name, cfg.env, cfg.host, cfg.port)
}
