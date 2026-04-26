pub fn init_config() -> String { "/cornbe/markdown".to_string() }
pub fn validate_input(path:&str)->Result<(),String>{ if path.is_empty(){Err("path empty".into())} else {Ok(())}}
pub fn execute_core(title:&str, body:&str)->String{ format!("<h1>{}</h1><pre>{}</pre>", title, body) }
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4911}else{5911}}
pub fn to_response(html:&str)->String{ html.to_string() }
