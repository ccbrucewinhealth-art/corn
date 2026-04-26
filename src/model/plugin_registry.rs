#[derive(Debug, Clone)]
pub struct PluginRegistry { pub plugin_id: String, pub lang: String, pub enabled: bool }

pub fn init_config() -> PluginRegistry { PluginRegistry { plugin_id:"p1".into(), lang:"python".into(), enabled:true } }
pub fn validate_input(v:&PluginRegistry)->Result<(),String>{ if v.plugin_id.is_empty(){Err("plugin_id empty".into())} else {Ok(())}}
pub fn execute_core(v:PluginRegistry)->Result<PluginRegistry,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4811}else{5811}}
pub fn to_response(v:&PluginRegistry)->String{ format!("{}:{}:{}", v.plugin_id,v.lang,v.enabled)}
