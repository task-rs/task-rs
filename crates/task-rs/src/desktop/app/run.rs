use super::super::super::components::Main;
use super::super::status::Status;
use super::App;
use iced::Application;

impl App {
    pub fn run(&self) -> Status<()> {
        Main::run(self.settings());
        Ok(())
    }
}
