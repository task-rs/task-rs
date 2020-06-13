use super::super::super::{
    config::Source as CfgSrc,
    data::Data,
    mvc::model::{Model, Progress, Title, View},
};
use super::App;
use iced::Settings;

impl App {
    pub(crate) fn settings(&self) -> Settings<Model> {
        let (config, config_source) = match self.config() {
            Ok((config, config_source)) => (Some(config), config_source),
            Err(error) => {
                eprintln!("WARN {}", error);
                (None, CfgSrc::default())
            }
        };
        let model = Model {
            config,
            config_source,
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
