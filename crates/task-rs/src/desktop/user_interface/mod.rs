pub mod data;

pub use data::Data;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInterface {
    pub state: Data,
}
