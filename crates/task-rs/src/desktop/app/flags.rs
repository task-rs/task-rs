use super::super::super::{
    data::Data,
    mvc::model::{Model, Progress, Title, View},
};
use super::App;

impl App {
    fn fallback_flags(&self) -> Model {
        let config = match self.config() {
            Ok(config) => Some(config),
            Err(error) => {
                eprintln!("WARN {}", error);
                None
            }
        };
        Model {
            config,
            data: Data::default(),
            progress: Progress::default(),
            title: Title::default(),
            view: View::default(),
        }
    }

    pub(crate) fn flags(&self) -> Model {
        self.fallback_flags()
    }
}
