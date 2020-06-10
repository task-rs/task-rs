use super::super::super::mvc::Model;
use super::super::status::{code, Status};
use super::{App, Args};
use iced::{Sandbox, Settings};
use pipe_trait::*;

impl App {
    pub fn init() -> Status<Self> {
        let args = Args::from_env()?;
        let app = App { args };
        Ok(app)
    }

    pub fn run(&self) -> Status<()> {
        let settings = Settings::default();
        Model::run(settings);
        Ok(())
    }

    pub fn main() -> i32 {
        Self::init().and_then(|app| app.run()).pipe(code)
    }
}
