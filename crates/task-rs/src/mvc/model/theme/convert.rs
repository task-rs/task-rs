use super::super::super::super::style::{Style, DARK, LIGHT};
use super::Theme;

impl Theme {
    pub fn style(&self) -> Style {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
