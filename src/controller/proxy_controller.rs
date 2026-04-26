use crate::service::proxy_service;

pub fn init_config() -> String { "/corn/api/0.85/proxy".into() }
pub fn validate_input(path:&str)->Result<(),String>{ proxy_service::validate_input(path) }
pub fn execute_core(path:&str)->Result<String,String>{
    let v = proxy_service::execute_core(path)?;
    Ok(proxy_service::to_response(&v))
}
pub fn map_error_code(err:&str)->i32{ proxy_service::map_error_code(err) }
pub fn to_response(v:&str)->String{ format!("{{"proxy":"{}"}}", v) }
