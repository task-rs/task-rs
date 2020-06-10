use super::super::super::mvc::Model;
use super::App;
use iced::{Sandbox, Settings};

impl App {
    pub fn run(&self) {
        let settings = Settings::default();
        Model::run(settings);
    }
}
