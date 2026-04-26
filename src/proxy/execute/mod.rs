use std::sync::Mutex;

use once_cell::sync::Lazy;
use crate::logging;
use super::common::{ProxyAction, ProxyConfig};

static ROUTE_CACHE: Lazy<Mutex<Vec<String>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Code-ID: SRC-040
/// 初始化 execute 模組。
pub fn init_config() -> String {
    "proxy-execute-ready".to_string()
}

/// Code-ID: SRC-040
/// 驗證 execute 輸入。
pub fn validate_input(cfg: &ProxyConfig, action: ProxyAction) -> Result<(), String> {
    if cfg.version != 3 {
        return Err(format!("unsupported proxy config version={}", cfg.version));
    }
    if matches!(action, ProxyAction::List | ProxyAction::Reload | ProxyAction::Health)
        && cfg.endpoints.is_empty()
    {
        return Err("proxy endpoints is empty".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-040
/// 執行 proxy 動作：list/reload/health。
pub fn execute_core(cfg: &ProxyConfig, action: ProxyAction) -> Result<String, String> {
    logging::debug(&format!(
        "[SRC-040] proxy execute_core action={} endpoints={}",
        action.as_str(),
        cfg.endpoints.len()
    ));
    validate_input(cfg, action)?;
    match action {
        ProxyAction::List => list_routes(cfg),
        ProxyAction::Reload => reload_routes(cfg),
        ProxyAction::Health => health_check(cfg),
    }
}

fn list_routes(cfg: &ProxyConfig) -> Result<String, String> {
    let cache = ROUTE_CACHE
        .lock()
        .map_err(|_| "proxy route cache poisoned".to_string())?;
    if !cache.is_empty() {
        logging::debug(&format!(
            "[SRC-040] proxy list use cache routes={}",
            cache.len()
        ));
        return Ok(format!("routes(cache): {}", cache.join("; ")));
    }

    let items = cfg
        .endpoints
        .iter()
        .map(|e| format!("{} {} backends={}", e.method, e.endpoint, e.backend.len()))
        .collect::<Vec<_>>()
        .join("; ");
    logging::debug(&format!(
        "[SRC-040] proxy list use config routes={}",
        cfg.endpoints.len()
    ));
    Ok(format!("routes(config): {}", items))
}

fn reload_routes(cfg: &ProxyConfig) -> Result<String, String> {
    let summaries = cfg
        .endpoints
        .iter()
        .enumerate()
        .map(|(idx, ep)| {
            let backend_hosts = ep
                .backend
                .iter()
                .flat_map(|b| b.host.iter().cloned())
                .collect::<Vec<_>>()
                .join(",");
            logging::debug(&format!(
                "[SRC-040] proxy reload route#{idx} method={} endpoint={} backends={} hosts=[{}]",
                ep.method,
                ep.endpoint,
                ep.backend.len(),
                backend_hosts
            ));
            format!("{} {} backends={}", ep.method, ep.endpoint, ep.backend.len())
        })
        .collect::<Vec<_>>();

    let mut cache = ROUTE_CACHE
        .lock()
        .map_err(|_| "proxy route cache poisoned".to_string())?;
    *cache = summaries;
    logging::debug(&format!(
        "[SRC-040] proxy reload done cache_routes={}",
        cache.len()
    ));
    Ok(format!("proxy reload ok endpoints={}", cfg.endpoints.len()))
}

fn health_check(cfg: &ProxyConfig) -> Result<String, String> {
    let cache_size = ROUTE_CACHE
        .lock()
        .map_err(|_| "proxy route cache poisoned".to_string())?
        .len();
    logging::debug(&format!(
        "[SRC-040] proxy health check endpoints={} cache_routes={}",
        cfg.endpoints.len(),
        cache_size
    ));
    Ok("proxy health=UP".to_string())
}

impl ProxyAction {
    fn as_str(&self) -> &'static str {
        match self {
            ProxyAction::List => "list",
            ProxyAction::Reload => "reload",
            ProxyAction::Health => "health",
        }
    }
}

pub fn map_error_code(err: &str) -> i32 {
    if err.contains("unsupported") || err.contains("empty") {
        4171
    } else {
        5171
    }
}

pub fn to_response(v: &str) -> String {
    v.to_string()
}
