use super::super::super::super::style;
use iced::{button, Background};
use pipe_trait::*;

pub struct StyleSheet {
    pub style: style::Style,
    pub activated: bool,
}
impl button::StyleSheet for StyleSheet {
    fn active(&self) -> button::Style {
        let StyleSheet { style, activated } = self;

        button::Style {
            background: if *activated {
                style.task_status_filter_activated_bg
            } else {
                style.task_status_filter_inactivated_bg
            }
            .pipe(Background::Color)
            .pipe(Some),
            text_color: if *activated {
                style.task_status_filter_activated_text
            } else {
                style.task_status_filter_inactivated_text
            },
            ..button::Style::default()
        }
    }
}
