use super::super::super::{
    data::Data,
    mvc::model::{Model, Progress, Title, View},
    utils::{deserialize_file, ui_state_file},
};
use super::App;

impl App {
    fn load_saved_state(&self) -> Result<Model, String> {
        match ui_state_file() {
            Some(ui_state_file) => deserialize_file(&ui_state_file),
            None => Err("Cannot determine ui state file location".to_owned()),
        }
    }

    fn fallback_state(&self) -> Model {
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
        self.load_saved_state().unwrap_or_else(|error| {
            eprintln!("WARN {}", error);
            self.fallback_state()
        })
    }
}
