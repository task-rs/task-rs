use super::super::status::Status;
use super::{App, Args};

impl App {
    pub fn init() -> Status<Self> {
        let args = Args::from_env()?;
        let app = App { args };
        Ok(app)
    }
}
