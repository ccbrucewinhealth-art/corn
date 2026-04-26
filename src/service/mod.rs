pub mod auth;
pub mod markdown_service;
pub mod proxy_service;
pub mod routes;

pub fn init_config() -> String { "service".into() }
pub fn validate_input(v:&str)->Result<(),String>{ if v.is_empty(){Err("service empty".into())} else {Ok(())}}
pub fn execute_core() -> String { format!("{} ready", init_config()) }
pub fn map_error_code(_err:&str)->i32{ 5030 }
pub fn to_response(msg:&str)->String{ msg.to_string() }
