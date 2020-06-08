use super::super::super::{config::Config, data::Manifest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Data {
    Unloaded,
    EditManifest { config: Config, manifest: Manifest },
    EditConfig { config: Option<Config> },
}
