use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Page {
    Tasks,
    #[default]
    Config,
    About,
}
