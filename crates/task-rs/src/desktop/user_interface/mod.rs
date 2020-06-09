pub mod data_state;
pub mod progress;
pub mod view;

pub use data_state::DataState;
pub use progress::Progress;
pub use view::View;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub progress: Progress,
    pub data_state: DataState,
    pub view: View,
}
