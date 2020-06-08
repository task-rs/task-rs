use super::super::super::{config::Config, data::Data as Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Data {
    Blank,
    EditManifest { config: Config, manifest: Database },
    EditConfig { config: Option<Config> },
}
