mod api;
mod config;
mod controller;
mod cron;
mod db;
mod executor;
mod logging;
mod model;
mod pages;
mod plugin;
mod service;
mod supervisor;
mod ui;
mod view;

pub fn init_config() -> Result<String, String> {
    let cfg = config::mod_::execute_core()?;
    Ok(config::mod_::to_response(&cfg))
}

pub fn validate_input() -> Result<(), String> { Ok(()) }

pub fn execute_core() -> Result<String, String> {
    validate_input()?;
    let summary = init_config()?;
    Ok(format!("corn-src runtime ready: {}", summary))
}

pub fn map_error_code(err: &str) -> i32 { if err.contains("invalid") { 4000 } else { 5000 } }

pub fn to_response(msg: &str) -> String { msg.to_string() }

fn main() {
    match execute_core() {
        Ok(v) => println!("{}", to_response(&v)),
        Err(e) => {
            eprintln!("error={}, code={}", e, map_error_code(&e));
            std::process::exit(1);
        }
    }
}
