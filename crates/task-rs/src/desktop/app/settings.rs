use super::super::super::components::Main;
use super::App;
use iced::Settings;

impl App {
    pub(crate) fn settings(&self) -> Settings<Main> {
        let mut settings = Settings::with_flags(self.flags());
        settings.antialiasing = true;
        settings
    }
}
