#[derive(Debug, Clone)]
pub struct ApiRoute {
    pub method: &'static str,
    pub url: &'static str,
    pub has_path_var: bool,
}

pub fn init_config() -> Vec<ApiRoute> {
    vec![
        ApiRoute { method: "GET", url: "/corn/api/0.85/proxy/backends", has_path_var: false },
        ApiRoute { method: "GET", url: "/corn/api/0.85/proxy/backends/{backendId}", has_path_var: true },
        ApiRoute { method: "PUT", url: "/corn/api/0.85/proxy/backends/{backendId}", has_path_var: true },
    ]
}

pub fn validate_input(route: &ApiRoute) -> Result<(), String> {
    if !route.url.starts_with('/') { return Err("url invalid".into()); }
    Ok(())
}

pub fn execute_core() -> Result<Vec<ApiRoute>, String> {
    let routes = init_config();
    for r in &routes { validate_input(r)?; }
    Ok(routes)
}

pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4080}else{5080}}

pub fn to_response(routes:&[ApiRoute])->String {
    let list: Vec<String> = routes.iter().map(|r| format!("{} {} var={}", r.method, r.url, r.has_path_var)).collect();
    list.join("
")
}
