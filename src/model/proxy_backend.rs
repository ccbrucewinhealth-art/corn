#[derive(Debug, Clone)]
pub struct ProxyBackend { pub backend_id: String, pub upstream: String, pub enabled: bool }

pub fn init_config() -> ProxyBackend { ProxyBackend { backend_id:"core".into(), upstream:"http://127.0.0.1:9000".into(), enabled:true } }
pub fn validate_input(v:&ProxyBackend)->Result<(),String>{ if !v.upstream.starts_with("http") {Err("upstream invalid".into())} else {Ok(())}}
pub fn execute_core(v:ProxyBackend)->Result<ProxyBackend,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4831}else{5831}}
pub fn to_response(v:&ProxyBackend)->String{ format!("{}:{}:{}", v.backend_id,v.upstream,v.enabled)}
