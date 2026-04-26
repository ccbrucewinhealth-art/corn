use std::fs;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::Duration;
use std::time::UNIX_EPOCH;

use crate::executor::{self, JobType};
use crate::logging;

static HELLO_EVERY_5_SEC_TASK: OnceLock<()> = OnceLock::new();
static CRON_SCAN_TASK: OnceLock<()> = OnceLock::new();
static CRON_JOB_REGISTRY: OnceLock<Mutex<std::collections::HashSet<String>>> = OnceLock::new();
static LAST_SOURCE_SIGNATURE: OnceLock<Mutex<Option<SourceSignature>>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct CronEntry {
    pub job_id: String,
    pub expr: String,
    pub command: String,
    pub host: String,
}

#[derive(Debug, Clone)]
pub struct CronRuntimeMeta {
    pub cron_file: String,
    pub cron_db: String,
    pub cron_jobs_table: String,
    pub cron_batch_jobs_table: String,
    pub cron_batch_period: u64,
    pub cron_scan_period: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SourceSignature {
    crontab_mtime: u64,
    jobs_seed_mtime: u64,
    batch_seed_mtime: u64,
}

/// Code-ID: SRC-003
/// 初始化 cron 相關配置並完成前置工作：
/// 1) 檢查/建立 `./.crontab`
/// 2) 準備 cronDefDatbase / cronJobsTable 建表 SQL（含 UUID 與 5 個預設欄位）
/// 3) 準備 cronBatchJobsTable 建表 SQL（含批次需求欄位）
pub fn init_config() -> String {
    logging::debug("[SRC-003] cron init_config start");
    let meta = load_runtime_meta();
    logging::debug(&format!(
        "[SRC-003] cron meta db={} jobs_table={} batch_table={} period={}s scan={}s",
        meta.cron_db,
        meta.cron_jobs_table,
        meta.cron_batch_jobs_table,
        meta.cron_batch_period,
        meta.cron_scan_period
    ));

    let _ = ensure_crontab_file(&meta.cron_file);
    let _ = ensure_cron_jobs_table_sql(&meta);
    let _ = ensure_cron_batch_jobs_table_sql(&meta);

    update_source_signature(capture_source_signature(&meta));
    ensure_scan_task(meta.cron_scan_period);

    logging::debug(&format!("[SRC-003] cron init_config done file={}", meta.cron_file));
    meta.cron_file
}

/// Code-ID: SRC-003
/// 驗證 crontab 單行內容。
pub fn validate_input(line: &str) -> Result<(), String> {
    if line.trim().is_empty() || line.trim().starts_with('#') {
        return Err("skip".to_string());
    }
    Ok(())
}

/// Code-ID: SRC-003
/// 核心執行：在真正解析前先完成 `.crontab` 與 DB 表前置檢查，並合併啟動來源。
pub fn execute_core(content: &str) -> Vec<CronEntry> {
    logging::debug(&format!(
        "[SRC-003] enter cron execute_core with content={} current_host={}",
        content,
        current_host()
    ));

    // 依需求：在 execute_core 前啟動背景 hello 任務（每 5 秒）
    ensure_hello_every_5_sec_task();

    let meta = load_runtime_meta();

    let _ = ensure_crontab_file(&meta.cron_file);
    let _ = ensure_cron_jobs_table_sql(&meta);
    let _ = ensure_cron_batch_jobs_table_sql(&meta);

    let current_signature = capture_source_signature(&meta);

    let mut merged = vec![];

    // 來源 A：傳入內容（例如 svc 啟動時注入）
    merged.extend(parse_entries(content, &current_host()));
    logging::debug(&format!(
        "[SRC-003] cron execute loaded source=injected entries={}",
        merged.len()
    ));
    log_merged_state("injected", &merged);

    // 來源 B：`.crontab`
    if let Ok(file_content) = fs::read_to_string(&meta.cron_file) {
        let entries = parse_entries(&file_content, &current_host());
        logging::debug(&format!(
            "[SRC-003] cron execute loaded source=file path={} entries={}",
            meta.cron_file,
            entries.len()
        ));
        merged.extend(entries);
        log_merged_state("file", &merged);
    }

    // 來源 C：模擬 DB (`cronDefTable` / `cronJobsTable`)
    if let Ok(db_entries) = load_mock_db_jobs(&meta) {
        logging::debug(&format!(
            "[SRC-003] cron execute loaded source=jobs_table entries={}",
            db_entries.len()
        ));
        merged.extend(db_entries);
        log_merged_state("jobs_table", &merged);
    }

    // 來源 D：模擬 DB (`cronBatchJobsTable`)
    if let Ok(batch_entries) = load_mock_db_batch_jobs(&meta) {
        logging::debug(&format!(
            "[SRC-003] cron execute loaded source=batch_table entries={}",
            batch_entries.len()
        ));
        merged.extend(batch_entries);
        log_merged_state("batch_table", &merged);
    }

    // 來源 E：啟動時預設批次流程（每 cronBatchPeriod 執行）
    let default_batch = load_default_batch_entries(&meta);
    logging::debug(&format!(
        "[SRC-003] cron execute loaded source=default_batch entries={}",
        default_batch.len()
    ));
    merged.extend(default_batch);
    log_merged_state("default_batch", &merged);

    // 去重策略：同鍵保留後者（DB/Batch 可覆蓋 .crontab）
    let deduped = dedup_keep_last(merged);
    logging::debug(&format!(
        "[SRC-003] cron execute done dedup_entries={}",
        deduped.len()
    ));

    schedule_jobs(&deduped);
    update_source_signature(current_signature);
    ensure_scan_task(meta.cron_scan_period);
    deduped
}

pub fn map_error_code(err: &str) -> i32 {
    if err == "skip" {
        1000
    } else {
        5000
    }
}

pub fn to_response(entries: &[CronEntry]) -> String {
    format!("loaded_entries={}", entries.len())
}

fn load_runtime_meta() -> CronRuntimeMeta {
    let cron_db = std::env::var("cronDefDatbase")
        .or_else(|_| std::env::var("cronDatbase"))
        .unwrap_or_else(|_| "TRC_Core".to_string());
    let cron_jobs_table = std::env::var("cronJobsTable").unwrap_or_else(|_| "cronDefTable".to_string());
    let cron_batch_jobs_table =
        std::env::var("cronBatchJobsTable").unwrap_or_else(|_| "cronBatchJobsTable".to_string());
    let cron_batch_period = std::env::var("cronBatchPeriod")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(60);
    let cron_scan_period = std::env::var("cronScanPeriod")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(15)
        .max(1);

    CronRuntimeMeta {
        cron_file: ".crontab".to_string(),
        cron_db,
        cron_jobs_table,
        cron_batch_jobs_table,
        cron_batch_period,
        cron_scan_period,
    }
}

fn ensure_crontab_file(path: &str) -> Result<(), String> {
    if Path::new(path).exists() {
        return Ok(());
    }
    let default = [
        "# corn default crontab",
        "# sec min hour day month weekday command",
        "*/15 * * * * * echo default-cron-from-file",
    ]
    .join("\n");
    fs::write(path, default).map_err(|e| e.to_string())
}

fn ensure_cron_jobs_table_sql(meta: &CronRuntimeMeta) -> Result<(), String> {
    let dir = Path::new("./sql/generated");
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }

    let sql = format!(
        "IF OBJECT_ID(N'{table}', N'U') IS NULL\nBEGIN\n  CREATE TABLE {table} (\n    Id uniqueidentifier NOT NULL DEFAULT NEWID() PRIMARY KEY,\n    CreatedBy nvarchar(64) NOT NULL DEFAULT N'system',\n    CreateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),\n    UpdatedBy nvarchar(64) NOT NULL DEFAULT N'system',\n    UpdateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),\n    Name nvarchar(200) NOT NULL DEFAULT N'default-job',\n    Host nvarchar(128) NOT NULL DEFAULT N'default',\n    CronExpr nvarchar(120) NOT NULL,\n    [Type] nvarchar(32) NOT NULL DEFAULT N'shell',\n    Content nvarchar(max) NULL,\n    Secret nvarchar(255) NULL,\n    IsEnable bit NOT NULL DEFAULT 1,\n    Concurrent bit NOT NULL DEFAULT 0,\n    DelayStart int NOT NULL DEFAULT 0\n  );\nEND;",
        table = meta.cron_jobs_table
    );
    fs::write(dir.join("001_cron_jobs_table.sql"), sql).map_err(|e| e.to_string())?;

    // mock DB seed（無真實資料庫時以檔案模擬）
    let mock_dir = Path::new("./data/mockdb");
    if !mock_dir.exists() {
        fs::create_dir_all(mock_dir).map_err(|e| e.to_string())?;
    }
    let seed = format!(
        "Id,Name,Host,CronExpr,Type,Content,Secret,IsEnable,Concurrent,DelayStart\n{id},default-db-job,{host},*/10 * * * * *,shell,echo from-db,,1,0,0\n",
        id = new_uuid(),
        host = current_host(),
    );
    let seed_path = mock_dir.join(format!("{}_seed.csv", meta.cron_jobs_table));
    if !seed_path.exists() {
        fs::write(seed_path, seed).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn ensure_cron_batch_jobs_table_sql(meta: &CronRuntimeMeta) -> Result<(), String> {
    let dir = Path::new("./sql/generated");
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }

    let sql = format!(
        "IF OBJECT_ID(N'{table}', N'U') IS NULL\nBEGIN\n  CREATE TABLE {table} (\n    Id uniqueidentifier NOT NULL DEFAULT NEWID() PRIMARY KEY,\n    CreatedBy nvarchar(64) NOT NULL DEFAULT N'system',\n    CreateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),\n    UpdatedBy nvarchar(64) NOT NULL DEFAULT N'system',\n    UpdateAt datetime2 NOT NULL DEFAULT SYSUTCDATETIME(),\n    Name nvarchar(200) NOT NULL DEFAULT N'default-batch-job',\n    JobId uniqueidentifier NULL,\n    Host nvarchar(128) NOT NULL DEFAULT N'default',\n    StartDateTime datetime2 NULL,\n    IsStarted bit NULL,\n    IsEnable bit NOT NULL DEFAULT 1,\n    [Once] bit NOT NULL DEFAULT 0,\n    WorkBeginAt nvarchar(64) NULL,\n    WorkEndAt nvarchar(64) NULL,\n    Concurrent bit NOT NULL DEFAULT 0,\n    DelayStart int NOT NULL DEFAULT 0,\n    Content nvarchar(max) NULL,\n    Result nvarchar(max) NULL,\n    Secret nvarchar(255) NULL,\n    [Type] nvarchar(32) NOT NULL DEFAULT N'shell'\n  );\nEND;",
        table = meta.cron_batch_jobs_table
    );

    fs::write(dir.join("002_cron_batch_jobs_table.sql"), sql).map_err(|e| e.to_string())?;

    // mock DB seed（無真實資料庫時以檔案模擬）
    let mock_dir = Path::new("./data/mockdb");
    if !mock_dir.exists() {
        fs::create_dir_all(mock_dir).map_err(|e| e.to_string())?;
    }
    let seed = format!(
        "Id,Name,JobId,Host,StartDateTime,IsStarted,IsEnable,Once,WorkBeginAt,WorkEndAt,Concurrent,DelayStart,Content,Result,Secret,Type\n{id},default-batch-job,,{host},,0,1,0,,,0,0,echo batch-from-db,,,shell\n",
        id = new_uuid(),
        host = current_host(),
    );
    let seed_path = mock_dir.join(format!("{}_seed.csv", meta.cron_batch_jobs_table));
    if !seed_path.exists() {
        fs::write(seed_path, seed).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn parse_entries(content: &str, host: &str) -> Vec<CronEntry> {
    content
        .lines()
        .filter_map(|line| {
            if validate_input(line).is_err() {
                return None;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 7 {
                return None;
            }
            Some(CronEntry {
                job_id: new_uuid(),
                expr: parts[..6].join(" "),
                command: parts[6..].join(" "),
                host: host.to_string(),
            })
        })
        .collect()
}

fn load_mock_db_jobs(meta: &CronRuntimeMeta) -> Result<Vec<CronEntry>, String> {
    let path = Path::new("./data/mockdb").join(format!("{}_seed.csv", meta.cron_jobs_table));
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut out = vec![];
    for (idx, line) in content.lines().enumerate() {
        if idx == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 6 {
            continue;
        }
        out.push(CronEntry {
            job_id: cols[0].to_string(),
            expr: cols[3].to_string(),
            command: if cols.len() > 5 {
                cols[5].to_string()
            } else {
                String::new()
            },
            host: cols[2].to_string(),
        });
    }
    Ok(out)
}

fn load_mock_db_batch_jobs(meta: &CronRuntimeMeta) -> Result<Vec<CronEntry>, String> {
    let path = Path::new("./data/mockdb").join(format!("{}_seed.csv", meta.cron_batch_jobs_table));
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let mut out = vec![];
    for (idx, line) in content.lines().enumerate() {
        if idx == 0 || line.trim().is_empty() {
            continue;
        }
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 16 {
            continue;
        }
        let is_started = parse_nullable_bool(cols[5]);
        let is_enable = parse_nullable_bool(cols[6]).unwrap_or(false);
        let once = parse_nullable_bool(cols[7]).unwrap_or(false);
        let concurrent = parse_nullable_bool(cols[10]).unwrap_or(false);
        let delay_start = cols[11].trim().parse::<u64>().unwrap_or(0);
        let work_begin_at = cols[8].trim();
        let work_end_at = cols[9].trim();
        let content = cols[12].trim();
        let secret = cols[14].trim();
        let job_type = cols[15].trim().to_ascii_lowercase();

        // IsStarted: null 表不啟用
        if is_started.is_none() {
            continue;
        }

        if !is_enable {
            continue;
        }

        // Once=true 且 IsStarted=false，視為已執行完成，不再執行
        if once && !is_started.unwrap_or(false) {
            continue;
        }

        // Concurrent=true 且 IsStarted=true，時間到也不啟用
        if concurrent && is_started.unwrap_or(false) {
            continue;
        }

        if !is_in_work_window(work_begin_at, work_end_at) {
            continue;
        }

        // secret 需與 cronExecSecret 相同才可執行
        if !is_secret_valid(secret) {
            continue;
        }

        // type 必須在支援清單
        if !matches!(job_type.as_str(), "shell" | "exec" | "python" | "js" | "sql") {
            continue;
        }

        let host = cols[3].trim();
        if !(host.eq_ignore_ascii_case(&current_host())
            || host.eq_ignore_ascii_case("default")
            || host == "*"
            || host.eq_ignore_ascii_case("all"))
        {
            continue;
        }

        // StartDateTime: null => 每 15 分鐘執行一次
        let expr = if cols[4].trim().is_empty() {
            "*/15 * * * * *".to_string()
        } else {
            format!("*/{} * * * * *", meta.cron_batch_period.max(1))
        };

        let command = if delay_start > 0 {
            format!("sleep {} && {}", delay_start, content)
        } else {
            content.to_string()
        };

        out.push(CronEntry {
            job_id: if cols[2].trim().is_empty() {
                cols[0].to_string()
            } else {
                cols[2].to_string()
            },
            expr,
            command,
            host: host.to_string(),
        });
    }
    Ok(out)
}

fn load_default_batch_entries(meta: &CronRuntimeMeta) -> Vec<CronEntry> {
    let expr = format!("*/{} * * * * *", meta.cron_batch_period.max(1));
    vec![CronEntry {
        job_id: new_uuid(),
        expr,
        command: format!(
            "batch-loader --db {} --table {}",
            meta.cron_db, meta.cron_batch_jobs_table
        ),
        host: current_host(),
    }]
}

// 取得當前主機識別（可透過環境變數 `cronCurrHost` 設定，預設為 "default"）
// 這樣可以模擬多主機環境下的行為，並在測試時輕鬆切換主機身份。
// 例如，在測試中可以設置 `cronCurrHost=host1` 或 `cronCurrHost=host2` 來模擬不同主機的行為。
// 這對於驗證 Host 欄位的過濾邏輯非常有幫助，確保只有符合當前主機識別的任務會被執行。
// 這也讓測試更具靈活性，無需修改程式碼即可模擬不同的主機環境。
// 在實際部署中，可以將 `cronCurrHost` 設置為具體的主機名稱或識別碼，以實現真正的多主機協同調度。
// 例如，在生產環境中，可以為不同的服務或實例設置不同的 `cronCurrHost`，以確保任務只在特定的主機上執行。
// 這種設計使得 cron 模組在多主機環境下更具適應性和可測試性，同時也提供了靈活的主機識別機制。
fn current_host() -> String {
    std::env::var("cronCurrHost").unwrap_or_else(|_| "default".to_string())
}

// 為了展示背景任務的運行，啟動一個每 5 秒打印一次點的任務。
fn ensure_hello_every_5_sec_task() {
    HELLO_EVERY_5_SEC_TASK.get_or_init(|| {
        thread::spawn(|| loop {
            println!("."); //print hello every 5 seconds to show the background task is running
            thread::sleep(Duration::from_secs(5));
        });
    });
}

fn parse_nullable_bool(raw: &str) -> Option<bool> {
    let s = raw.trim().to_ascii_lowercase();
    if s.is_empty() || s == "null" {
        return None;
    }
    match s.as_str() {
        "1" | "true" | "yes" | "y" => Some(true),
        "0" | "false" | "no" | "n" => Some(false),
        _ => None,
    }
}

fn is_secret_valid(secret: &str) -> bool {
    let expect = std::env::var("cronExecSecret").unwrap_or_default();
    if expect.trim().is_empty() {
        return true;
    }
    secret == expect
}

fn is_in_work_window(work_begin_at: &str, work_end_at: &str) -> bool {
    let now = chrono::Utc::now();

    // 規格文字提到 ISO8859，實作層以 RFC3339（ISO-8601）解析，無法解析時視為不限制
    if !work_begin_at.trim().is_empty() {
        if let Ok(begin) = chrono::DateTime::parse_from_rfc3339(work_begin_at) {
            if now < begin.with_timezone(&chrono::Utc) {
                return false;
            }
        }
    }

    if !work_end_at.trim().is_empty() {
        if let Ok(end) = chrono::DateTime::parse_from_rfc3339(work_end_at) {
            if now > end.with_timezone(&chrono::Utc) {
                return false;
            }
        }
    }

    true
}

fn dedup_keep_last(entries: Vec<CronEntry>) -> Vec<CronEntry> {
    let mut keys = std::collections::HashMap::<String, usize>::new();
    let mut out = Vec::<CronEntry>::new();
    for entry in entries {
        let key = format!("{}|{}|{}", entry.expr, entry.command, entry.host);
        if let Some(index) = keys.get(&key).copied() {
            out[index] = entry;
        } else {
            keys.insert(key, out.len());
            out.push(entry);
        }
    }
    out
}

fn log_merged_state(stage: &str, merged: &[CronEntry]) {
    let preview = merged
        .iter()
        .map(|e| format!("{}|{}|{}", e.job_id, e.expr, e.command))
        .collect::<Vec<_>>()
        .join("; ");
    logging::debug(&format!(
        "[SRC-003] cron merged stage={} merged_len={} merged={}",
        stage,
        merged.len(),
        preview
    ));
}

fn schedule_jobs(entries: &[CronEntry]) {
    let registry = CRON_JOB_REGISTRY.get_or_init(|| Mutex::new(std::collections::HashSet::new()));

    logging::debug(&format!(
        "[SRC-003] schedule_jobs start scheduling entries={} current_host={}",
        entries.len(),
        current_host()
    ));
    for entry in entries {
        let schedule_key = format!("{}|{}|{}", entry.expr, entry.command, entry.host);
        logging::debug(&format!(
            "[SRC-003] entry setting schedule_key={}",
            schedule_key
        ));
        let mut guard = registry.lock().expect("cron registry lock poisoned");
        if guard.contains(&schedule_key) {
            logging::debug(&format!(
                "[SRC-003] cron schedule skip existing key={}",
                schedule_key
            ));
            continue;
        }

        logging::debug(&format!(
            "[SRC-003] entry do insert schedule_key={}",
            schedule_key
        ));
        guard.insert(schedule_key.clone());
        drop(guard);

        let entry_clone = entry.clone();
        let interval_secs = parse_interval_seconds(&entry_clone.expr).unwrap_or(60).max(1);

        logging::debug(&format!(
            "[SRC-003] cron schedule set key={} expr={} interval_secs={} command={}",
            schedule_key, entry_clone.expr, interval_secs, entry_clone.command
        ));

        thread::spawn(move || {
            logging::debug(&format!(
                "[SRC-003] cron schedule start job_id={} expr={} interval_secs={}",
                entry_clone.job_id, entry_clone.expr, interval_secs
            ));

            loop {
                logging::debug(&format!(
                    "[SRC-003] cron schedule tick job_id={} command={}",
                    entry_clone.job_id, entry_clone.command
                ));

                match executor::execute_core(JobType::Shell, &entry_clone.command) {
                    Ok(output) => logging::debug(&format!(
                        "[SRC-003] cron schedule exec ok job_id={} output={}",
                        entry_clone.job_id, output
                    )),
                    Err(err) => logging::error(&format!(
                        "[SRC-003] cron schedule exec failed job_id={} err={}",
                        entry_clone.job_id, err
                    )),
                }

                thread::sleep(Duration::from_secs(interval_secs));
            }
        });
    }
}

fn parse_interval_seconds(expr: &str) -> Option<u64> {
    let first = expr.split_whitespace().next()?;
    if let Some(raw) = first.strip_prefix("*/") {
        return raw.parse::<u64>().ok();
    }
    if first == "*" {
        return Some(1);
    }
    first.parse::<u64>().ok().filter(|v| *v > 0)
}

fn capture_source_signature(meta: &CronRuntimeMeta) -> SourceSignature {
    let jobs_seed = Path::new("./data/mockdb").join(format!("{}_seed.csv", meta.cron_jobs_table));
    let batch_seed = Path::new("./data/mockdb").join(format!("{}_seed.csv", meta.cron_batch_jobs_table));

    SourceSignature {
        crontab_mtime: path_mtime_secs(Path::new(&meta.cron_file)),
        jobs_seed_mtime: path_mtime_secs(&jobs_seed),
        batch_seed_mtime: path_mtime_secs(&batch_seed),
    }
}

fn path_mtime_secs(path: &Path) -> u64 {
    fs::metadata(path)
        .ok()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn update_source_signature(signature: SourceSignature) {
    let last = LAST_SOURCE_SIGNATURE.get_or_init(|| Mutex::new(None));
    let mut guard = last.lock().expect("source signature lock poisoned");
    *guard = Some(signature);
}

fn has_source_changed(signature: &SourceSignature) -> bool {
    let last = LAST_SOURCE_SIGNATURE.get_or_init(|| Mutex::new(None));
    let guard = last.lock().expect("source signature lock poisoned");
    match &*guard {
        None => true,
        Some(prev) => prev != signature,
    }
}

fn ensure_scan_task(scan_period_secs: u64) {
    CRON_SCAN_TASK.get_or_init(|| {
        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(scan_period_secs.max(1)));

            let meta = load_runtime_meta();
            let current = capture_source_signature(&meta);
            if !has_source_changed(&current) {
                continue;
            }

            logging::info(&format!(
                "[SRC-003] detect cron source changed by cronScanPeriod={}s, reload scheduling",
                scan_period_secs
            ));

            let entries = execute_core("");
            for e in &entries {
                logging::info(&format!(
                    "[SRC-003] reload job id={} expr={} host={} command={}",
                    e.job_id, e.expr, e.host, e.command
                ));
            }
        });
    });
}

fn new_uuid() -> String {
    fs::read_to_string("/proc/sys/kernel/random/uuid")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| format!("fallback-{}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()))
}
