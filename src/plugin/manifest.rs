#[derive(Debug, Clone)]
pub struct Manifest { pub plugin_id: String, pub lang: String, pub version: String, pub entry: String }

pub fn init_config() -> Manifest { Manifest { plugin_id:"demo".into(), lang:"python".into(), version:"1.0.0".into(), entry:"main.py".into() } }
pub fn validate_input(v:&Manifest)->Result<(),String>{
    if v.plugin_id.is_empty() { return Err("plugin_id empty".into()); }
    if !(v.lang=="python" || v.lang=="javascript") { return Err("lang invalid".into()); }
    Ok(())
}
pub fn execute_core(v:Manifest)->Result<Manifest,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4941}else{5941}}
pub fn to_response(v:&Manifest)->String{ format!("{}:{}:{}", v.plugin_id,v.lang,v.version)}
