use crate::model::proxy_backend::ProxyBackend;

pub fn init_config() -> Vec<ProxyBackend> {
    vec![ProxyBackend{ backend_id:"core".into(), upstream:"http://127.0.0.1:9000".into(), enabled:true }]
}
pub fn validate_input(path:&str)->Result<(),String>{ if !path.starts_with('/') {Err("path invalid".into())} else {Ok(())}}
pub fn execute_core(path:&str)->Result<Option<ProxyBackend>,String>{
    validate_input(path)?;
    let list = init_config();
    Ok(list.into_iter().find(|x| x.enabled))
}
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4030}else{5030}}
pub fn to_response(v:&Option<ProxyBackend>)->String{ format!("matched={}", v.is_some()) }
