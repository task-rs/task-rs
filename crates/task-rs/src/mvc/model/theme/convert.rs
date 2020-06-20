use super::super::super::super::style::{self, Style, DARK, LIGHT};
use super::Theme;

impl style::Theme for Theme {
    fn style(&self) -> Style {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
