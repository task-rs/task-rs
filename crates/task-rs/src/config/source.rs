use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::path::PathBuf;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Source {
    #[default]
    ConfigFile(PathBuf),
    CommandLineArguments,
}
