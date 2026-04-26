use chrono::Local;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

/// Code-ID: SRC-013
/// 初始化日誌等級（R1 預設為 Info）。
pub fn init_config() -> LogLevel {
    LogLevel::Info
}

/// Code-ID: SRC-013
/// 驗證 log message 是否可用。
pub fn validate_input(message: &str) -> Result<(), String> {
    if message.trim().is_empty() {
        return Err("log message empty".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-013
/// 核心格式化：輸出時間戳 + level + message。
pub fn execute_core(level: LogLevel, message: &str) -> Result<String, String> {
    validate_input(message)?;
    let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
    Ok(format!("[{ts}] [{:?}] {}", level, message))
}

/// Code-ID: SRC-013
/// 錯誤訊息映射為錯誤碼。
pub fn map_error_code(err: &str) -> i32 {
    if err.contains("empty") {
        4701
    } else {
        5701
    }
}

/// Code-ID: SRC-013
/// 回傳可直接輸出的字串。
pub fn to_response(line: &str) -> String {
    line.to_string()
}

/// Code-ID: SRC-013
/// Step 訊息格式化，例如：Step.1.2 初始化設定。
pub fn step(no: u32, sub: u32, message: &str) -> String {
    format!("Step.{no}.{sub} {message}")
}

/// Code-ID: SRC-013
/// 統一輸出 debug log。
pub fn debug(message: &str) {
    emit(LogLevel::Debug, message);
}

/// Code-ID: SRC-013
/// 統一輸出 info log。
pub fn info(message: &str) {
    emit(LogLevel::Info, message);
}

/// Code-ID: SRC-013
/// 統一輸出 warn log。
pub fn warn(message: &str) {
    emit(LogLevel::Warn, message);
}

/// Code-ID: SRC-013
/// 統一輸出 error log。
pub fn error(message: &str) {
    emit(LogLevel::Error, message);
}

/// Code-ID: SRC-013
/// 統一輸出 fatal log。
pub fn fatal(message: &str) {
    emit(LogLevel::Fatal, message);
}

/// Code-ID: SRC-013
/// 內部輸出實作。
fn emit(level: LogLevel, message: &str) {
    match execute_core(level, message) {
        Ok(line) => println!("{}", to_response(&line)),
        Err(err) => {
            let code = map_error_code(&err);
            println!("[LOG-{}] {:?}: {}", code, level, err);
        }
    }
}
