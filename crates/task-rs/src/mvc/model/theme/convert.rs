use super::super::super::super::styling::{Styling, DARK, LIGHT};
use super::Theme;

impl Theme {
    pub fn color_scheme(&self) -> Styling {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
