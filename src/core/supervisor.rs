use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatus {
    pub name: String,
    pub pid: Option<u32>,
    pub state: String,
    pub uptime_sec: u64,
}

pub fn status() -> Result<Vec<ProcessStatus>> {
    Ok(vec![ProcessStatus {
        name: "corn-svc".to_string(),
        pid: Some(std::process::id()),
        state: "RUNNING".to_string(),
        uptime_sec: 0,
    }])
}

pub fn start() -> Result<()> {
    println!("[supervisor] start embedded process manager");
    Ok(())
}

pub fn stop() -> Result<()> {
    println!("[supervisor] stop embedded process manager");
    Ok(())
}

pub fn restart() -> Result<()> {
    stop()?;
    start()?;
    Ok(())
}

