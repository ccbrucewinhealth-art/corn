use std::sync::RwLock;

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::config::AppConfig;
use crate::logging;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub job_id: String,
    pub cron_expr: String,
    pub enabled: bool,
    pub job_type: String,
}

static JOBS: Lazy<RwLock<Vec<Job>>> = Lazy::new(|| {
    RwLock::new(vec![Job {
        job_id: "demo-job-001".to_string(),
        cron_expr: "0 */5 * * * *".to_string(),
        enabled: true,
        job_type: "shell".to_string(),
    }])
});

pub async fn bootstrap(_cfg: &AppConfig) -> Result<()> {
    logging::info(&logging::step(5, 1, "scheduler bootstrap"));
    Ok(())
}

pub async fn reload(_cfg: &AppConfig) -> Result<()> {
    logging::info(&logging::step(5, 2, "scheduler reload"));
    Ok(())
}

pub async fn list_jobs(_cfg: &AppConfig) -> Result<Vec<Job>> {
    logging::debug("[SRC-006] scheduler list_jobs called");
    let jobs = JOBS.read().expect("jobs lock poisoned");
    Ok(jobs.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_list_default_demo_job() {
        let cfg = AppConfig {
            app_name: "corn".to_string(),
            app_env: "test".to_string(),
            app_port: 8080,
            app_host: "127.0.0.1".to_string(),
            api_token: None,
            db_url: None,
            markdown_root: "./corn/data/markdown".to_string(),
            markdown_history_root: "./corn/data/markdown_history".to_string(),
            ui_template_root: "./corn/ui/templates".to_string(),
            ui_assets_root: "./corn/ui/assets".to_string(),
            plugin_root: "./corn/plugins".to_string(),
            plugin_table: "CornPluginRegistry".to_string(),
            supervisor_mode: "embedded".to_string(),
            proxy_bind: "0.0.0.0:8090".to_string(),
            api_prefix: "/corn/api/0.85".to_string(),
            ui_prefix: "/cornbe".to_string(),
            swagger_prefix: "/swagger".to_string(),
            assets_prefix: "/assets".to_string(),
            health_path: "/health".to_string(),
            proxy_plugin_db_url: None,
            proxy_plugin_table: "ProxyPluginCode".to_string(),
            proxy_plugin_fetch_cmd: None,
            ui_view_root_uri: "/corn/ui".to_string(),
            api_root: "/corn/api/0.85".to_string(),
            api_common_token: Some("tacos-common-token".to_string()),
            ui_path: "cornbe".to_string(),
            tacos_adminlte_template_root: "./bin/deploy/ui/templates/adminlte".to_string(),
            tacos_view_root: "./bin/deploy/ui/view".to_string(),
            tacos_model_root: "./bin/deploy/ui/model".to_string(),
            tacos_controller_root: "./bin/deploy/ui/controller".to_string(),
            tacos_api_root: "./bin/deploy/ui/api".to_string(),
            tacos_router_file: "./bin/deploy/ui/router.lua".to_string(),
            user_permission_file: "./conf/CornUsePermission.json".to_string(),
        };

        let jobs = list_jobs(&cfg).await.expect("list jobs should pass");
        assert!(!jobs.is_empty());
        assert_eq!(jobs[0].job_id, "demo-job-001");
    }
}
