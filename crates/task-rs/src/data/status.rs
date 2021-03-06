use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    #[default]
    Active,
    Completed,
}
