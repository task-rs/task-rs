use super::super::super::mvc::Model;
use super::super::status::{code, Status};
use super::{App, Args};
use iced::{window, Application, Settings};
use pipe_trait::*;

impl App {
    pub fn init() -> Status<Self> {
        let args = Args::from_env()?;
        let app = App { args };
        Ok(app)
    }

    pub fn settings(&self) -> Settings<Model> {
        Settings {
            antialiasing: true,
            default_font: None,
            flags: Model::default(),
            window: window::Settings::default(),
        }
    }

    pub fn run(&self) -> Status<()> {
        Model::run(self.settings());
        Ok(())
    }

    pub fn main() -> i32 {
        Self::init().and_then(|app| app.run()).pipe(code)
    }
}
