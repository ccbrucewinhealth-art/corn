use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "corn", version, about = "Rust Cron Jobs Core")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Start,
    Stop,
    Restart,
    Reload,
    List,
    Help,
    Svc {
        #[arg(long, default_value = "0.0.0.0:8080")]
        bind: String,
    },
    Proxy,
    Plugin {
        #[arg(value_enum)]
        action: PluginAction,
    },
    Supervisor {
        #[arg(value_enum)]
        action: SupervisorAction,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum PluginAction {
    List,
    Sync,
    Validate,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SupervisorAction {
    Status,
    Start,
    Stop,
    Restart,
}

