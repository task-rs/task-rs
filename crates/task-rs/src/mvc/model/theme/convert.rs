use super::super::super::super::style::{theme, Style, DARK, LIGHT};
use super::Theme;

impl theme::Theme for Theme {
    fn style(&self) -> Style {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}
