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
    components::Controls,
    config::{Config, Source as CfgSrc},
    data::Data,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub ui_state: UiState,
    pub data: Data,
    #[serde(skip_serializing, skip_deserializing)]
    pub controls: Controls,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct UiState {
    pub title: Title,
    #[serde(skip_serializing, skip_deserializing)]
    pub progress: Progress,
    pub view: View,
    pub config: Option<(Config, CfgSrc)>,
    pub theme: Theme,
    pub details: Details,
}
