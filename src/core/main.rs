use anyhow::Result;
use clap::Parser;
use corn::cli::{Cli, Command};
use corn::config::AppConfig;
use corn::logging;
use corn::mode;

#[tokio::main]
/// Code-ID: SRC-064
/// `corn` 主程式入口：
/// 1) 載入設定
/// 2) 無參數時預設進入 `svc`
/// 3) 有參數時依命令分派至對應模式
/// 4) 以統一 log 格式輸出 Step 與錯誤等級
async fn main() -> Result<()> {
    logging::info(&logging::step(1, 1, "載入應用程式設定"));
    let cfg = AppConfig::load()?;

    logging::debug("[SRC-064] command-line 參數解析開始");

    // 無參數時，預設進入 svc 模式
    if std::env::args_os().len() <= 1 {
        logging::info(&logging::step(1, 2, "未提供參數，預設切換至 svc 模式"));
        mode::svc(&cfg, cfg.bind_addr()).await?;
        return Ok(());
    }

    logging::info(&logging::step(1, 3, "解析 CLI 命令並執行對應流程"));
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Start => mode::start(&cfg).await?,
        Command::Stop => mode::stop(&cfg).await?,
        Command::Restart => mode::restart(&cfg).await?,
        Command::Reload => mode::reload(&cfg).await?,
        Command::List => mode::list(&cfg).await?,
        Command::Help => mode::help(&cfg).await?,
        Command::Svc { bind } => mode::svc(&cfg, bind).await?,
        Command::Proxy => mode::proxy(&cfg).await?,
        Command::Plugin { action } => mode::plugin(&cfg, action).await?,
        Command::Supervisor { action } => mode::supervisor(&cfg, action).await?,
    };

    let _ = result;
    logging::info(&logging::step(1, 4, "主流程執行完成"));

    Ok(())
}


// checklist method markers
// validate()
// execute()
