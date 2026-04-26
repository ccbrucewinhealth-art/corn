use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::proxy_mod::{common::ProxyAction, execute, parser};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyRoute {
    pub route_id: String,
    pub path_prefix: String,
    pub upstream: String,
    pub timeout_ms: u64,
    pub enabled: bool,
}

pub fn load_routes() -> Result<Vec<ProxyRoute>> {
    let cfg_path = parser::init_config();
    let cfg = parser::execute_core(&cfg_path).map_err(anyhow::Error::msg)?;

    // 啟動時做一次 health execute，對齊 F17 parser + execute 流程
    let _ = execute::execute_core(&cfg, ProxyAction::Health).map_err(anyhow::Error::msg)?;

    let mut out = vec![];
    for (idx, ep) in cfg.endpoints.iter().enumerate() {
        let upstream = ep
            .backend
            .first()
            .and_then(|b| b.host.first())
            .cloned()
            .unwrap_or_else(|| "http://127.0.0.1:9000".to_string());
        out.push(ProxyRoute {
            route_id: if idx == 0 {
                "core-api".to_string()
            } else {
                format!("route-{}", idx)
            },
            path_prefix: ep.endpoint.clone(),
            upstream,
            timeout_ms: 15_000,
            enabled: true,
        });
    }

    if out.is_empty() {
        out.push(ProxyRoute {
            route_id: "core-api".to_string(),
            path_prefix: "/api/core".to_string(),
            upstream: "http://127.0.0.1:9000".to_string(),
            timeout_ms: 15_000,
            enabled: true,
        });
    }

    Ok(out)
}
