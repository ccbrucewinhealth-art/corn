use serde::{Deserialize, Serialize};

/// Code-ID: SRC-040
/// F17 proxy 支援動作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProxyAction {
    List,
    Reload,
    Health,
}

/// Code-ID: SRC-040
/// KrakenD 風格 backend 定義（R1 縮減版）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyBackendConfig {
    pub host: Vec<String>,
    pub url_pattern: String,
    pub encoding: Option<String>,
}

/// Code-ID: SRC-040
/// KrakenD 風格 endpoint 定義（R1 縮減版）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyEndpointConfig {
    pub endpoint: String,
    pub method: String,
    pub output_encoding: Option<String>,
    pub backend: Vec<ProxyBackendConfig>,
}

/// Code-ID: SRC-040
/// F17 主設定物件。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub schema: Option<String>,
    pub version: u8,
    pub endpoints: Vec<ProxyEndpointConfig>,
}

impl ProxyConfig {
    pub fn default_minimal() -> Self {
        Self {
            schema: Some("https://www.krakend.io/schema/v2.13/krakend.json".to_string()),
            version: 3,
            endpoints: vec![ProxyEndpointConfig {
                endpoint: "/api/core".to_string(),
                method: "GET".to_string(),
                output_encoding: Some("json".to_string()),
                backend: vec![ProxyBackendConfig {
                    host: vec!["http://127.0.0.1:9000".to_string()],
                    url_pattern: "/core".to_string(),
                    encoding: Some("json".to_string()),
                }],
            }],
        }
    }
}
