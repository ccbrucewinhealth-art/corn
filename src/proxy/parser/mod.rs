use std::fs;
use std::path::Path;

use serde_json::Value;

use crate::logging;
use super::common::{ProxyBackendConfig, ProxyConfig, ProxyEndpointConfig};

/// Code-ID: SRC-040
/// 取得 KrakenD 設定檔路徑。
pub fn init_config() -> String {
    let path = std::env::var("cronProxyConfigFile")
        .unwrap_or_else(|_| "./bin/deploy/conf/krakend.json".to_string());
    logging::debug(&format!("[SRC-040] proxy parser init_config path={}", path));
    path
}

/// Code-ID: SRC-040
/// 驗證 parser 輸入。
pub fn validate_input(path: &str) -> Result<(), String> {
    if path.trim().is_empty() {
        return Err("proxy config path is empty".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-040
/// 解析 KrakenD JSON 設定（R1: version/endpoints/backend/extra_config 可忽略）。
pub fn execute_core(path: &str) -> Result<ProxyConfig, String> {
    logging::debug(&format!(
        "[SRC-040] proxy parser execute_core loading path={}",
        path
    ));
    validate_input(path)?;
    if !Path::new(path).exists() {
        logging::debug(
            "[SRC-040] proxy parser config file not found, fallback to default_minimal",
        );
        return Ok(ProxyConfig::default_minimal());
    }

    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    logging::debug(&format!(
        "[SRC-040] proxy parser config file loaded bytes={}",
        content.len()
    ));
    parse_from_json(&content)
}

pub fn map_error_code(err: &str) -> i32 {
    if err.contains("empty") || err.contains("version") {
        4170
    } else {
        5170
    }
}

pub fn to_response(cfg: &ProxyConfig) -> String {
    format!("version={} endpoints={}", cfg.version, cfg.endpoints.len())
}

fn parse_from_json(content: &str) -> Result<ProxyConfig, String> {
    let root: Value = serde_json::from_str(content).map_err(|e| e.to_string())?;

    let version = root
        .get("version")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "krakend version missing".to_string())? as u8;

    if version != 3 {
        return Err(format!("unsupported krakend config version={version}, expect=3"));
    }

    let schema = root
        .get("$schema")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut endpoints = vec![];
    if let Some(arr) = root.get("endpoints").and_then(|v| v.as_array()) {
        for ep in arr {
            let endpoint = ep
                .get("endpoint")
                .and_then(|v| v.as_str())
                .unwrap_or("/")
                .to_string();
            let method = ep
                .get("method")
                .and_then(|v| v.as_str())
                .unwrap_or("GET")
                .to_string();
            let output_encoding = ep
                .get("output_encoding")
                .and_then(|v| v.as_str())
                .map(|v| v.to_string());

            let mut backends = vec![];
            if let Some(be_arr) = ep.get("backend").and_then(|v| v.as_array()) {
                for be in be_arr {
                    let url_pattern = be
                        .get("url_pattern")
                        .and_then(|v| v.as_str())
                        .unwrap_or("/")
                        .to_string();
                    let host = be
                        .get("host")
                        .and_then(|v| v.as_array())
                        .map(|hs| {
                            hs.iter()
                                .filter_map(|h| h.as_str().map(|s| s.to_string()))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let encoding = be
                        .get("encoding")
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string());
                    if !host.is_empty() {
                        backends.push(ProxyBackendConfig {
                            host,
                            url_pattern,
                            encoding,
                        });
                    }
                }
            }

            endpoints.push(ProxyEndpointConfig {
                endpoint,
                method,
                output_encoding,
                backend: backends,
            });
        }
    }

    Ok(ProxyConfig {
        schema,
        version,
        endpoints,
    })
}
