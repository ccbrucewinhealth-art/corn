pub fn init_config() -> String { "ui".into() }
pub fn validate_input(page:&str)->Result<(),String>{ if page.is_empty(){Err("page empty".into())} else {Ok(())}}
pub fn execute_core(page:&str)->Result<String,String>{ validate_input(page)?; Ok(format!("render page={}", page)) }
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4060}else{5060}}
pub fn to_response(v:&str)->String{ v.to_string() }
