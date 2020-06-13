use super::super::super::{
    data::Data,
    mvc::model::{Model, Progress, Title, View},
};
use super::App;
use iced::Settings;

impl App {
    pub(crate) fn settings(&self) -> Settings<Model> {
        let config = match self.config() {
            Ok(config) => Some(config),
            Err(error) => {
                eprintln!("WARN {}", error);
                None
            }
        };
        let model = Model {
            config,
            data: Data::default(),
            progress: Progress::default(),
            title: Title::default(),
            view: View::default(),
        };
        let mut settings = Settings::with_flags(model);
        settings.antialiasing = true;
        settings
    }
}
