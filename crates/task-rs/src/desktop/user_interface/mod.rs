pub mod config;
pub mod progress;
pub mod view;

pub use config::Config;
pub use progress::Progress;
pub use view::View;

use super::super::data::Data;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub progress: Progress,
    pub view: View,
    pub data: Data,
    pub config: Config,
}
