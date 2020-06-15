pub mod details;
pub mod progress;
pub mod theme;
pub mod title;
pub mod view;

pub use details::Details;
pub use progress::Progress;
pub use theme::Theme;
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
    pub ui_state: UiState,
    pub data: Data,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UiState {
    pub title: Title,
    pub progress: Progress,
    pub view: View,
    pub config: Option<(Config, CfgSrc)>,
    pub theme: Theme,
    pub details: Details,
}
