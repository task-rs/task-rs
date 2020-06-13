use super::super::super::mvc::Model;
use super::App;
use iced::Settings;

impl App {
    pub(crate) fn settings(&self) -> Settings<Model> {
        let mut settings = Settings::with_flags(Model::default());
        settings.antialiasing = true;
        settings
    }
}
