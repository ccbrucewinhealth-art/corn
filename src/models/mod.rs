#[derive(Debug, Clone)]
pub struct CommonResult<T> { pub code: i32, pub message: String, pub data: T }

pub fn init_config() -> CommonResult<()> { CommonResult { code:0, message:"ok".into(), data:() } }
pub fn validate_input<T>(_v:&CommonResult<T>)->Result<(),String>{ Ok(()) }
pub fn execute_core<T>(v:CommonResult<T>)->Result<CommonResult<T>,String>{ Ok(v) }
pub fn map_error_code(_err:&str)->i32{ 5900 }
pub fn to_response<T>(_v:&CommonResult<T>)->String{ "common-result".into() }
