use super::super::super::super::{
    components::task_status_filter,
    style::{Style, DARK, LIGHT},
};
use super::Theme;

impl Theme {
    pub fn style(&self) -> Style {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
        }
    }
}

impl task_status_filter::Theme for &Theme {
    fn style(&self) -> Style {
        Theme::style(self)
    }
}
