#[derive(Debug, Clone)]
pub struct LoginRequest { pub username: String, pub password: String }

pub fn init_config() -> &'static str { "admin" }
pub fn validate_input(req:&LoginRequest)->Result<(),String>{ if req.username.is_empty() || req.password.is_empty(){Err("credential empty".into())} else {Ok(())}}
pub fn execute_core(req:LoginRequest)->Result<String,String>{
    validate_input(&req)?;
    if req.username=="admin" && req.password=="admin" { Ok("token-demo".into()) } else { Err("invalid credential".into()) }
}
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4010}else{5010}}
pub fn to_response(token:&str)->String{ format!("Bearer {}", token) }
