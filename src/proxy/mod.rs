use crate::logging;

pub mod common;
pub mod execute;
pub mod parser;

/// Code-ID: SRC-040
/// 初始化 proxy 模組預設設定（回傳 enabled）。
pub fn init_config() -> String {
    std::env::var("cronProxyEnabled").unwrap_or_else(|_| "false".to_string())
}

/// Code-ID: SRC-040
/// 驗證 proxy enabled 參數。
pub fn validate_input(enabled: &str) -> Result<(), String> {
    match enabled.to_ascii_lowercase().as_str() {
        "true" | "false" => Ok(()),
        _ => Err(format!("invalid cronProxyEnabled: {enabled}")),
    }
}

/// Code-ID: SRC-040
/// proxy 背景初始化核心流程（svc 相關）。
/// 1) 載入 parser 設定檔路徑
/// 2) 解析 proxy 設定
/// 3) 執行 health 動作
pub fn execute_core() -> Result<String, String> {
    logging::debug("[SRC-040] Step.2.1 proxy::execute_core start");

    let enabled = init_config();
    validate_input(&enabled)?;
    if enabled.eq_ignore_ascii_case("false") {
        logging::debug("[SRC-040] Step.2.1 proxy::execute_core skipped by cronProxyEnabled=false");
        return Ok("proxy execute_core skipped enabled=false".to_string());
    }

    let cfg_path = parser::init_config();
    logging::debug(&format!(
        "[SRC-040] Step.2.2 proxy parser init_config path={}",
        cfg_path
    ));

    let cfg = parser::execute_core(&cfg_path).unwrap_or_else(|err| {
        logging::warn(&format!(
            "[SRC-040] Step.2.3 proxy parser fallback default_minimal err={}",
            err
        ));
        common::ProxyConfig::default_minimal()
    });

    logging::debug(&format!(
        "[SRC-040] Step.2.3 proxy parser loaded endpoints={}",
        cfg.endpoints.len()
    ));

    let reload_output = execute::execute_core(&cfg, common::ProxyAction::Reload)?;
    logging::debug(&format!(
        "[SRC-040] Step.2.4 proxy execute action=reload output={}",
        reload_output
    ));

    let output = execute::execute_core(&cfg, common::ProxyAction::Health)?;
    logging::debug(&format!(
        "[SRC-040] Step.2.5 proxy execute action=health output={}",
        output
    ));

    Ok(format!(
        "proxy execute_core ok enabled={} endpoints={} reload={} output={}",
        enabled,
        cfg.endpoints.len(),
        reload_output,
        output
    ))
}
