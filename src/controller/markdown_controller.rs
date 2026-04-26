use crate::service::markdown_service;

pub fn init_config() -> String { "/corn/api/0.85/md".into() }
pub fn validate_input(path:&str)->Result<(),String>{ markdown_service::validate_input(path) }
pub fn execute_core(path:&str, content:&str)->Result<String,String>{
    markdown_service::execute_core(path, content)?;
    Ok(markdown_service::to_response(path))
}
pub fn map_error_code(err:&str)->i32{ markdown_service::map_error_code(err) }
pub fn to_response(v:&str)->String{ format!("{{"message":"{}"}}", v) }
