pub mod progress;
pub mod title;
pub mod view;

pub use progress::Progress;
pub use title::Title;
pub use view::View;

use super::super::{
    config::{Config, Source as CfgSrc},
    data::Data,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub title: Title,
    pub progress: Progress,
    pub view: View,
    pub data: Data,
    pub config: Option<(Config, CfgSrc)>,
}
