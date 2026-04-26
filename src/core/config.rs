use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::{cron, executor, proxy_mod, supervisor_mod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub app_env: String,
    pub app_port: u16,
    pub app_host: String,
    pub api_token: Option<String>,
    pub db_url: Option<String>,
    pub markdown_root: String,
    pub markdown_history_root: String,
    pub ui_template_root: String,
    pub ui_assets_root: String,
    pub plugin_root: String,
    pub plugin_table: String,
    pub supervisor_mode: String,
    pub proxy_bind: String,
    pub api_prefix: String,
    pub ui_prefix: String,
    pub swagger_prefix: String,
    pub assets_prefix: String,
    pub health_path: String,
    pub proxy_plugin_db_url: Option<String>,
    pub proxy_plugin_table: String,
    pub proxy_plugin_fetch_cmd: Option<String>,
    pub ui_view_root_uri: String,
    pub api_root: String,
    pub api_common_token: Option<String>,
    pub ui_path: String,
    pub tacos_adminlte_template_root: String,
    pub tacos_view_root: String,
    pub tacos_model_root: String,
    pub tacos_controller_root: String,
    pub tacos_api_root: String,
    pub tacos_router_file: String,
    pub user_permission_file: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let _ = dotenvy::from_path("../.env");
        let _ = dotenvy::dotenv();

        Ok(Self {
            app_name: env_str("CORN_APP_NAME", "corn"),
            app_env: env_str("CORN_APP_ENV", "dev"),
            app_port: env_u16("CORN_APP_PORT", 8080)?,
            app_host: env_str("CORN_APP_HOST", "0.0.0.0"),
            api_token: std::env::var("CORN_API_TOKEN").ok(),
            db_url: std::env::var("CORN_DB_URL").ok(),
            markdown_root: env_str("CORN_MD_ROOT", "./corn/data/markdown"),
            markdown_history_root: env_str("CORN_MD_HISTORY_ROOT", "./corn/data/markdown_history"),
            ui_template_root: env_str("CORN_UI_TEMPLATE_ROOT", "./corn/ui/templates"),
            ui_assets_root: env_str("CORN_UI_ASSETS_ROOT", "./corn/ui/assets"),
            plugin_root: env_str("CORN_PLUGIN_ROOT", "./corn/plugins"),
            plugin_table: env_str("CORN_PLUGIN_TABLE", "CornPluginRegistry"),
            supervisor_mode: env_str("CORN_SUPERVISOR_MODE", "embedded"),
            proxy_bind: env_str("CORN_PROXY_BIND", "0.0.0.0:8090"),
            api_prefix: env_str("CORN_API_PREFIX", "/corn/api/0.85"),
            ui_prefix: env_str("CORN_UI_PREFIX", "/cornbe"),
            swagger_prefix: env_str("CORN_SWAGGER_PREFIX", "/swagger"),
            assets_prefix: env_str("CORN_ASSETS_PREFIX", "/assets"),
            health_path: env_str("CORN_HEALTH_PATH", "/health"),
            proxy_plugin_db_url: std::env::var("CORN_PROXY_PLUGIN_DB_URL")
                .ok()
                .or_else(|| std::env::var("CORN_DB_URL_CORE").ok()),
            proxy_plugin_table: env_str("CORN_PROXY_PLUGIN_TABLE", "ProxyPluginCode"),
            proxy_plugin_fetch_cmd: std::env::var("CORN_PROXY_PLUGIN_FETCH_CMD").ok(),
            ui_view_root_uri: env_str("CORN_UI_VIEW_ROOT", "/corn/ui"),
            api_root: env_str("CORN_API_ROOT", "/corn/api/0.85"),
            api_common_token: std::env::var("CORN_API_COMMON_TOKEN").ok(),
            ui_path: env_str("CORN_UI_PATH", "cornbe"),
            tacos_adminlte_template_root: env_str(
                "CORN_TACOS_ADMINLTE_TEMPLATE_ROOT",
                "./bin/deploy/ui/templates/adminlte",
            ),
            tacos_view_root: env_str("CORN_TACOS_VIEW_ROOT", "./bin/deploy/ui/view"),
            tacos_model_root: env_str("CORN_TACOS_MODEL_ROOT", "./bin/deploy/ui/model"),
            tacos_controller_root: env_str(
                "CORN_TACOS_CONTROLLER_ROOT",
                "./bin/deploy/ui/controller",
            ),
            tacos_api_root: env_str("CORN_TACOS_API_ROOT", "./bin/deploy/ui/api"),
            tacos_router_file: env_str("CORN_TACOS_ROUTER_FILE", "./bin/deploy/ui/router.lua"),
            user_permission_file: env_str("CORN_USER_PERMISSION_FILE", "./conf/CornUsePermission.json"),
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    /// Code-ID: SRC-002
    /// 初始化 svc 相關相依模組設定：
    /// - cron::init_config
    /// - executor::init_config
    /// - supervisor_mod::init_config
    /// - proxy_mod::parser::init_config
    pub fn init_config_svc_rel(&self) -> Result<String> {
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

        supervisor_mod::validate_input(&supervisor_cfg)
            .map_err(|e| anyhow::anyhow!(e))?;

        proxy_mod::parser::validate_input(&proxy_cfg_path)
            .map_err(|e| anyhow::anyhow!(e))?;

        crate::logging::debug("[SRC-002] init_config_svc_rel done");

        Ok(format!(
            "svc-rel initialized: cron={}, executor_kinds={}, supervisor_mode={}, proxy_cfg={}",
            cron_cfg,
            executor_cfg.len(),
            supervisor_cfg,
            proxy_cfg_path,
        ))
    }
}

fn env_str(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_u16(key: &str, default: u16) -> Result<u16> {
    match std::env::var(key) {
        Ok(v) => v
            .parse::<u16>()
            .with_context(|| format!("invalid {}={}", key, v)),
        Err(_) => Ok(default),
    }
}
