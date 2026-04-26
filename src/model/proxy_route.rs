#[derive(Debug, Clone)]
pub struct ProxyRoute { pub route_id: String, pub path: String, pub method: String }

pub fn init_config() -> ProxyRoute { ProxyRoute { route_id:"r1".into(), path:"/api/core".into(), method:"GET".into() } }
pub fn validate_input(v:&ProxyRoute)->Result<(),String>{ if !v.path.starts_with('/') {Err("path invalid".into())} else {Ok(())}}
pub fn execute_core(v:ProxyRoute)->Result<ProxyRoute,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4841}else{5841}}
pub fn to_response(v:&ProxyRoute)->String{ format!("{}:{}:{}", v.route_id,v.method,v.path)}
