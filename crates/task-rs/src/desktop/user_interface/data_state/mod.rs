use super::super::super::{config::Config, data::Data};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataState {
    Blank,
    EditManifest { config: Config, data: Data },
    EditConfig { config: Option<Config> },
}
