use super::super::super::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Data {
    Unloaded,
    Loaded { config: Config },
}
