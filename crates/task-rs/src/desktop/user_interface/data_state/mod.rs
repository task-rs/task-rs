use super::super::super::{config::Config, data::Data};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataState {
    Blank,
    EditManifest { config: Config, data: Data },
    EditConfig { config: Option<Config> },
}

impl Default for DataState {
    fn default() -> Self {
        DataState::Blank
    }
}
