use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cron-cli", version, about = "AI CLI tool for corn")]
struct Cli {
    #[arg(long, env = "CORN_API_BASE", default_value = "http://127.0.0.1:8080")]
    api_base: String,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Ask { prompt: String },
    Jobs,
    Plugins,
    MarkdownTree {
        #[arg(long, default_value = "")]
        dir: String,
    },
}

fn main() -> Result<()> {
    let _ = dotenvy::from_path("../.env");
    let _ = dotenvy::dotenv();
    let cli = Cli::parse();

    match cli.command {
        Command::Ask { prompt } => {
            println!(
                "[cron-cli] mock route => POST {}/corn/api/0.85/ai/ask body={{\"prompt\":\"{}\"}}",
                cli.api_base, prompt
            );
        }
        Command::Jobs => {
            println!("[cron-cli] mock route => GET {}/corn/api/0.85/jobs", cli.api_base);
        }
        Command::Plugins => {
            println!(
                "[cron-cli] mock route => GET {}/corn/api/0.85/plugin/list",
                cli.api_base
            );
        }
        Command::MarkdownTree { dir } => {
            println!(
                "[cron-cli] mock route => GET {}/corn/api/0.85/md/tree?dir={}",
                cli.api_base, dir
            );
        }
    }

    Ok(())
}



// checklist method markers
// load()
// validate()
// execute()
