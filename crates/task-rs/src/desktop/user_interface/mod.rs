pub mod data_state;
pub mod view;

pub use data_state::DataState;
pub use view::View;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub data_state: DataState,
    pub view: View,
}
