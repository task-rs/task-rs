use super::super::super::mvc::Model;
use super::super::status::Status;
use super::App;
use iced::Application;

impl App {
    pub fn run(&self) -> Status<()> {
        Model::run(self.settings());
        Ok(())
    }
}
