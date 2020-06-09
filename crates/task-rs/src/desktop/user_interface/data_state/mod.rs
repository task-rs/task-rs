use super::super::super::{config::Config, data::Data, default_enum};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataState {
    Blank,
    EditManifest { config: Config, data: Data },
    EditConfig { config: Option<Config> },
}

default_enum!(DataState::Blank);
