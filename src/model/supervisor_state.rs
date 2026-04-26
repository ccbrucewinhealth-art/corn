#[derive(Debug, Clone)]
pub struct SupervisorState { pub mode: String, pub running: bool, pub process_count: usize }

pub fn init_config() -> SupervisorState { SupervisorState { mode:"embedded".into(), running:true, process_count:1 } }
pub fn validate_input(v:&SupervisorState)->Result<(),String>{ if v.mode.is_empty(){Err("mode empty".into())} else {Ok(())}}
pub fn execute_core(v:SupervisorState)->Result<SupervisorState,String>{ validate_input(&v)?; Ok(v) }
pub fn map_error_code(err:&str)->i32{ if err.contains("empty"){4851}else{5851}}
pub fn to_response(v:&SupervisorState)->String{ format!("{}:{}:{}", v.mode,v.running,v.process_count)}
