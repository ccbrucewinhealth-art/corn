pub fn init_config() -> String { "/cornbe/proxy-backends".to_string() }
pub fn validate_input(query:&str)->Result<(),String>{ if query.len()>2048{Err("query too long".into())} else {Ok(())}}
pub fn execute_core(rows:&[(&str,&str)]) -> String {
    let mut out = String::from("<table><tr><th>id</th><th>upstream</th></tr>");
    for (id,up) in rows { out.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", id, up)); }
    out.push_str("</table>");
    out
}
pub fn map_error_code(err:&str)->i32{ if err.contains("long"){4921}else{5921}}
pub fn to_response(html:&str)->String{ html.to_string() }
