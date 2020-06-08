pub mod data;

pub use data::Data;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub data: Data,
}
