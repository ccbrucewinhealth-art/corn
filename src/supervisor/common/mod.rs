use std::collections::HashMap;

/// Code-ID: SRC-045
/// Supervisor 支援動作。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupervisorAction {
    Status,
    Start,
    Stop,
    Restart,
}

impl SupervisorAction {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Restart => "restart",
        }
    }
}

/// Code-ID: SRC-045
/// 解析後的 supervisor 設定。
#[derive(Debug, Clone)]
pub struct SupervisorConfig {
    pub mode: String,
    pub ctl_path: String,
    pub conf_file: String,
    pub program_name: String,
    pub unix_socket: String,
    pub ctl_timeout_sec: u64,
    pub enabled: bool,
    pub program_entries: Vec<SupervisorProgramEntry>,
    pub sections: HashMap<String, HashMap<String, String>>,
}

/// Code-ID: SRC-045
/// 從 supervisord 設定檔解析出的 program 定義。
#[derive(Debug, Clone)]
pub struct SupervisorProgramEntry {
    pub name: String,
    pub command: String,
    pub autostart: bool,
}

impl SupervisorConfig {
    pub fn from_env_defaults() -> Self {
        Self {
            mode: std::env::var("cronSupervisorMode")
                .or_else(|_| std::env::var("CORN_SUPERVISOR_MODE"))
                .unwrap_or_else(|_| "embedded".to_string()),
            ctl_path: std::env::var("cronSupervisorCtlPath")
                .unwrap_or_else(|_| "/usr/bin/supervisorctl".to_string()),
            conf_file: std::env::var("cronSupervisorConfFile")
                .unwrap_or_else(|_| "/etc/supervisor/supervisord.conf".to_string()),
            program_name: std::env::var("cronSupervisorProgramName").unwrap_or_else(|_| "corn".to_string()),
            unix_socket: std::env::var("cronSupervisorUnixSocket")
                .unwrap_or_else(|_| "/var/run/supervisor.sock".to_string()),
            ctl_timeout_sec: std::env::var("cronSupervisorCtlTimeoutSec")
                .ok()
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(10),
            enabled: std::env::var("cronSupervisorEnabled")
                .ok()
                .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes"))
                .unwrap_or(false),
            program_entries: vec![],
            sections: HashMap::new(),
        }
    }
}
