#[derive(Debug, Clone)]
pub struct PluginVersion { pub plugin_id: String, pub version: String, pub entry: String }

pub fn init_config() -> PluginVersion { PluginVersion { plugin_id:"p1".into(), version:"1.0.0".into(), entry:"main.py".into() } }
pub fn validate_input(v:&PluginVersion)->Result<(),String>{ if v.version.is_empty(){Err("version empty".into())} else {Ok(())}}
pub fn execute_core(v:PluginVersion)->Result<PluginVersion,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4821}else{5821}}
pub fn to_response(v:&PluginVersion)->String{ format!("{}:{}:{}", v.plugin_id,v.version,v.entry)}
