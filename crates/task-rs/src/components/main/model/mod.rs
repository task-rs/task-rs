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

use super::super::super::{
    config::{Config, Source as CfgSrc},
    data::Data,
};
use super::super::Controls;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Model {
    pub ui_state: UiState,
    pub data: Data,
    #[serde(skip)]
    pub controls: Controls,
}

impl Clone for Model {
    fn clone(&self) -> Self {
        Model {
            data: self.data.clone(),
            ui_state: self.ui_state.clone(),
            controls: Controls::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct UiState {
    pub title: Title,
    #[serde(skip)]
    pub progress: Progress,
    pub view: View,
    pub config: Option<(Config, CfgSrc)>,
    pub theme: Theme,
    pub details: Details,
}
