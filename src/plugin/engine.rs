use std::fs;
use std::path::Path;

pub fn init_config() -> String { "plugins".into() }
pub fn validate_input(path:&str)->Result<(),String>{ if path.trim().is_empty(){Err("plugin path empty".into())} else {Ok(())}}
pub fn execute_core(plugin_root:&str) -> Result<Vec<String>, String> {
    validate_input(plugin_root)?;
    let p = Path::new(plugin_root);
    if !p.exists() { return Ok(vec![]); }
    let mut out = vec![];
    for e in fs::read_dir(p).map_err(|e| e.to_string())? {
        let e = e.map_err(|e| e.to_string())?;
        if e.path().is_dir() { out.push(e.file_name().to_string_lossy().to_string()); }
    }
    Ok(out)
}
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4931}else{5931}}
pub fn to_response(v:&[String])->String{ format!("plugins={}", v.len()) }
