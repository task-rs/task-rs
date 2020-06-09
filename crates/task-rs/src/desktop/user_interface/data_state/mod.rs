use super::super::super::{config::Config, data::Data};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DataState {
    #[default]
    Blank,
    EditManifest {
        config: Config,
        data: Data,
    },
    EditConfig {
        config: Option<Config>,
    },
}
