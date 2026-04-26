use std::fs;
use std::path::PathBuf;

pub fn init_config() -> PathBuf { PathBuf::from("./corn/data/markdown") }
pub fn validate_input(path:&str)->Result<(),String>{ if path.contains("..") {Err("path traversal".into())} else {Ok(())}}
pub fn execute_core(path:&str, content:&str)->Result<(),String>{
    validate_input(path)?;
    let root = init_config();
    let target = root.join(path);
    if let Some(parent)=target.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    fs::write(target, content).map_err(|e| e.to_string())
}
pub fn map_error_code(err:&str)->i32{ if err.contains("traversal"){4020}else{5020}}
pub fn to_response(path:&str)->String{ format!("written={}", path) }
