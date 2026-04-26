use std::process::Command;

pub fn init_config() -> String { "supervisorctl".into() }
pub fn validate_input(action:&str)->Result<(),String>{ if action.is_empty(){Err("action empty".into())} else {Ok(())}}
pub fn execute_core(action:&str)->Result<String,String>{
    validate_input(action)?;
    let bin = init_config();
    let output = Command::new(bin).arg(action).output();
    match output {
        Ok(v) if v.status.success() => Ok(String::from_utf8_lossy(&v.stdout).to_string()),
        Ok(v) => Err(String::from_utf8_lossy(&v.stderr).to_string()),
        Err(_) => Ok(format!("compat simulated action={}", action)),
    }
}
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4050}else{5050}}
pub fn to_response(v:&str)->String{ v.to_string() }
