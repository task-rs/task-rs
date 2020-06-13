use super::super::status::code;
use super::App;
use pipe_trait::*;

impl App {
    pub fn main() -> i32 {
        Self::init().and_then(|app| app.run()).pipe(code)
    }
}
