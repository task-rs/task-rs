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
                style.activated_button_bg
            } else {
                style.inactivated_button_bg
            }
            .pipe(Background::Color)
            .pipe(Some),
            text_color: if *activated {
                style.activated_button_text
            } else {
                style.inactivated_button_text
            },
            ..button::Style::default()
        }
    }
}
