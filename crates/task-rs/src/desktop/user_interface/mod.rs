pub mod progress;
pub mod view;

pub use progress::Progress;
pub use view::View;

use super::super::{
    config::{Config, Source as CfgSrc},
    data::Data,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UserInterface {
    pub progress: Progress,
    pub view: View,
    pub data: Data,
    pub config: Option<Config>,
    pub config_source: CfgSrc,
}
