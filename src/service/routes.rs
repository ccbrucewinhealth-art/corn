#[derive(Debug, Clone)]
pub struct Route { pub method: &'static str, pub path: &'static str }

pub fn init_config() -> Vec<Route> {
    vec![
        Route{method:"GET", path:"/health"},
        Route{method:"GET", path:"/corn/api/0.85/jobs"},
        Route{method:"GET", path:"/corn/api/0.85/plugin/list"},
    ]
}
pub fn validate_input(route:&Route)->Result<(),String>{ if !route.path.starts_with('/') {Err("path invalid".into())} else {Ok(())}}
pub fn execute_core() -> Vec<Route> { init_config() }
pub fn map_error_code(err:&str)->i32{ if err.contains("invalid"){4040}else{5040}}
pub fn to_response(routes:&[Route])->String{ format!("routes={}", routes.len()) }
