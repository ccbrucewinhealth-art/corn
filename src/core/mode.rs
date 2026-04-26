use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;

use crate::api;
use crate::cli::{PluginAction, SupervisorAction};
use crate::config::AppConfig;
use crate::cron;
use crate::executor::{self, JobType};
use crate::logging;
use crate::plugin;
use crate::proxy;
use crate::proxy_mod;
use crate::scheduler;
use crate::supervisor;
use crate::supervisor_mod;

/// Code-ID: SRC-064
/// 啟動流程：初始化排程並輸出啟動資訊。
pub async fn start(cfg: &AppConfig) -> Result<()> {
    logging::info(&logging::step(2, 1, "啟動 start 模式"));
    scheduler::bootstrap(cfg).await?;
    logging::info(&format!("[SRC-064] start ok env={}", cfg.app_env));
    Ok(())
}

/// Code-ID: SRC-064
/// 停止流程：記錄 stop 要求。
pub async fn stop(_cfg: &AppConfig) -> Result<()> {
    logging::warn("[SRC-064] stop requested");
    Ok(())
}

/// Code-ID: SRC-064
/// 重啟流程：先 stop 再 start。
pub async fn restart(cfg: &AppConfig) -> Result<()> {
    logging::info(&logging::step(2, 2, "執行 restart：stop -> start"));
    stop(cfg).await?;
    start(cfg).await?;
    Ok(())
}

/// Code-ID: SRC-064
/// 重載流程：重新載入排程配置。
pub async fn reload(cfg: &AppConfig) -> Result<()> {
    scheduler::reload(cfg).await?;
    logging::info("[SRC-064] reload ok");
    Ok(())
}

/// Code-ID: SRC-064
/// 查詢流程：列出目前 job 清單。
pub async fn list(cfg: &AppConfig) -> Result<()> {
    logging::info(&logging::step(2, 3, "列出排程工作清單"));
    let jobs = scheduler::list_jobs(cfg).await?;
    for j in jobs {
        println!("{}\t{}\t{}", j.job_id, j.cron_expr, j.enabled);
    }
    Ok(())
}

/// Code-ID: SRC-064
/// 說明流程：輸出可用命令列表。
pub async fn help(_cfg: &AppConfig) -> Result<()> {
    println!(
        "corn commands: start|stop|restart|reload|list|help|svc|proxy|plugin|supervisor"
    );
    Ok(())
}

/// Code-ID: SRC-064
/// 服務流程：啟動 API 服務端。
pub async fn svc(cfg: &AppConfig, bind: String) -> Result<()> {
    logging::info(&logging::step(3, 1, &format!("啟動 svc 模式 bind={bind}")));
    logging::info(&logging::step(3, 2, "載入 svc 相關設定"));
    let svc_rel = cfg.init_config_svc_rel()?;
    logging::info(&format!("[SRC-064] {}", svc_rel));

    // 背景執行 svc 相關初始化工作
    logging::info(&logging::step(3, 3, "啟動背景初始化工作（cron/executor/supervisor）"));
    tokio::spawn(async move {
        // cron::mode execute_core（由 .crontab + DB + batch 預設來源整合）
        let cron_result = cron::execute_core("");
        logging::info(&format!(
            "[SRC-064] Step.3.3 cron::execute_core loaded_entries={}",
            cron_result.len()
        ));

        // executor::mode execute_core
        match executor::execute_core(JobType::Shell, "echo svc-init") {
            Ok(output) => logging::info(&format!(
                "[SRC-064] Step.3.4 executor::execute_core ok output={}",
                output
            )),
            Err(err) => logging::error(&format!(
                "[SRC-064] Step.3.4 executor::execute_core failed err={}",
                err
            )),
        }

        // supervisor::mod execute_core
        match supervisor_mod::execute_core() {
            Ok(output) => logging::info(&format!(
                "[SRC-064] Step.3.5 supervisor::execute_core ok output={}",
                output
            )),
            Err(err) => logging::error(&format!(
                "[SRC-064] Step.3.5 supervisor::execute_core failed err={}",
                err
            )),
        }

        // proxy::mod execute_core
        match proxy_mod::execute_core() {
            Ok(output) => logging::info(&format!(
                "[SRC-064] Step.3.6 proxy::execute_core ok output={}",
                output
            )),
            Err(err) => logging::error(&format!(
                "[SRC-064] Step.3.6 proxy::execute_core failed err={}",
                err
            )),
        }
    });

    // 背景執行 cronExecAtStart（若有設定）
    logging::info(&logging::step(3, 7, "檢查 cronExecAtStart 背景啟動設定"));
    spawn_exec_at_start_task();

    api::run_server(cfg, &bind).await
}

/// Code-ID: SRC-064
/// 在 svc 啟動前，背景觸發 `cronExecAtStart` 指定程式：
/// - 空值：不執行
/// - `.sh`：以 `bashPath`（預設 `/bin/bash`）執行
/// - 非 `.sh`：直接執行
/// - stdout/stderr：沿用目前程序（inherit）
fn spawn_exec_at_start_task() {
    let exec_path = std::env::var("cronExecAtStart").unwrap_or_default();
    let exec_path = exec_path.trim().to_string();

    if exec_path.is_empty() {
        logging::debug("[SRC-064] Step.3.6 cronExecAtStart is empty, skip");
        return;
    }

    tokio::spawn(async move {
        logging::info(&format!(
            "[SRC-064] Step.3.6 cronExecAtStart detected path={}",
            exec_path
        ));

        let mut cmd = if exec_path.ends_with(".sh") {
            let bash_path = std::env::var("bashPath").unwrap_or_else(|_| "/bin/bash".to_string());
            let mut command = Command::new(bash_path);
            command.arg(&exec_path);
            command
        } else {
            Command::new(&exec_path)
        };

        cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

        match cmd.spawn() {
            Ok(mut child) => match child.wait().await {
                Ok(status) => {
                    if status.success() {
                        logging::info(&format!(
                            "[SRC-064] Step.3.6 cronExecAtStart finished status={}",
                            status
                        ));
                    } else {
                        logging::warn(&format!(
                            "[SRC-064] Step.3.6 cronExecAtStart non-zero status={}",
                            status
                        ));
                    }
                }
                Err(err) => logging::error(&format!(
                    "[SRC-064] Step.3.6 cronExecAtStart wait failed err={}",
                    err
                )),
            },
            Err(err) => logging::error(&format!(
                "[SRC-064] Step.3.6 cronExecAtStart spawn failed err={}",
                err
            )),
        }
    });
}

/// Code-ID: SRC-064
/// Proxy 流程：載入反向代理路由並輸出摘要。
pub async fn proxy(cfg: &AppConfig) -> Result<()> {
    let routes = proxy::load_routes()?;
    logging::info(&format!(
        "[SRC-064] reverse proxy bind={} routes={}",
        cfg.proxy_bind,
        routes.len()
    ));
    Ok(())
}

/// Code-ID: SRC-064
/// Plugin 流程：依 action 執行 list/sync/validate。
pub async fn plugin(cfg: &AppConfig, action: PluginAction) -> Result<()> {
    match action {
        PluginAction::List => {
            logging::debug("[SRC-064] plugin action=list");
            for p in plugin::scan_plugins(cfg)? {
                println!("{}\t{}\t{}", p.plugin_id, p.lang, p.version);
            }
        }
        PluginAction::Sync => {
            logging::info(&logging::step(3, 2, "執行 plugin sync"));
            let count = plugin::sync_registry(cfg).await?;
            logging::info(&format!("[SRC-064] plugin sync ok count={count}"));
        }
        PluginAction::Validate => {
            logging::info(&logging::step(3, 3, "執行 plugin validate"));
            plugin::validate_all(cfg)?;
            logging::info("[SRC-064] plugin validate ok");
        }
    }
    Ok(())
}

/// Code-ID: SRC-064
/// Supervisor 流程：執行 status/start/stop/restart。
pub async fn supervisor(cfg: &AppConfig, action: SupervisorAction) -> Result<()> {
    let mode = cfg.supervisor_mode.as_str();
    match action {
        SupervisorAction::Status => {
            let list = supervisor::status()?;
            logging::info(&format!("[SRC-064] supervisor({mode}) processes={}", list.len()));
        }
        SupervisorAction::Start => {
            supervisor::start()?;
            logging::info(&format!("[SRC-064] supervisor({mode}) start"));
        }
        SupervisorAction::Stop => {
            supervisor::stop()?;
            logging::warn(&format!("[SRC-064] supervisor({mode}) stop"));
        }
        SupervisorAction::Restart => {
            supervisor::restart()?;
            logging::info(&format!("[SRC-064] supervisor({mode}) restart"));
        }
    }
    Ok(())
}
