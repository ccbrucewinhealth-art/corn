use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct JobRow {
    pub job_id: String,
    pub cron: String,
    pub enabled: bool,
}

#[derive(Clone)]
pub struct Db {
    inner: Arc<RwLock<Vec<JobRow>>>,
}

pub fn init_config() -> Db {
    Db { inner: Arc::new(RwLock::new(vec![JobRow { job_id: "demo-job".to_string(), cron: "0 */5 * * * *".to_string(), enabled: true }])) }
}

pub fn validate_input(job: &JobRow) -> Result<(), String> {
    if job.job_id.is_empty() { return Err("job_id empty".to_string()); }
    Ok(())
}

pub fn execute_core(db: &Db) -> Vec<JobRow> {
    db.inner.read().expect("db lock").clone()
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("empty") { 4201 } else { 5201 } }

pub fn to_response(rows: &[JobRow]) -> String { format!("rows={}", rows.len()) }
