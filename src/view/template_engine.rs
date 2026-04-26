use std::fs;
use std::path::Path;

pub fn init_config() -> String { "./corn/ui/templates".into() }
pub fn validate_input(name:&str)->Result<(),String>{ if !name.ends_with(".html"){Err("template invalid".into())} else {Ok(())}}
pub fn execute_core(name:&str)->Result<String,String>{
    validate_input(name)?;
    let p = Path::new(&init_config()).join(name);
    fs::read_to_string(p).map_err(|e| e.to_string())
}
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4070}else{5070}}
pub fn to_response(v:&str)->String{ v.to_string() }
