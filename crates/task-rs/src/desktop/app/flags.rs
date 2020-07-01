use super::super::super::{
    components::{
        main::model::{Model, UiState},
        Controls,
    },
    utils::{deserialize_file, load_data_from_cfg_opt, ui_state_file},
};
use super::App;

impl App {
    fn load_saved_state(&self) -> Result<UiState, String> {
        match ui_state_file() {
            Some(ui_state_file) => deserialize_file(&ui_state_file),
            None => Err("Cannot determine ui state file location".to_owned()),
        }
    }

    fn fallback_state(&self) -> UiState {
        let config = match self.config() {
            Ok(config) => Some(config),
            Err(error) => {
                eprintln!("WARN {}", error);
                None
            }
        };
        UiState {
            config,
            ..UiState::default()
        }
    }

    pub(crate) fn flags(&self) -> Model {
        let ui_state = self.load_saved_state().unwrap_or_else(|error| {
            eprintln!("WARN {}", error);
            self.fallback_state()
        });
        let data = load_data_from_cfg_opt(&ui_state.config);
        Model {
            ui_state,
            data,
            controls: Controls::default(),
        }
    }
}
