pub mod progress;
pub mod view;

pub use progress::Progress;
pub use view::View;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub progress: Progress,
    pub view: View,
}
