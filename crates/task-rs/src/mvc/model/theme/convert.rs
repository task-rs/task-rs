use super::super::super::super::style::{Style, DARK, LIGHT};
use super::Theme;

impl Theme {
    pub fn color_scheme(&self) -> Style {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
