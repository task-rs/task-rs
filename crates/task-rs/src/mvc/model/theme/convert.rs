use super::super::super::super::color::{ColorScheme, DARK, LIGHT};
use super::Theme;

impl Theme {
    pub fn color_scheme(&self) -> ColorScheme {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
