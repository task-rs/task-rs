pub type Status<Value> = Result<Value, i32>;

pub fn code<Value>(status: Status<Value>) -> i32 {
    if let Err(code) = status {
        code
    } else {
        0
    }
}
