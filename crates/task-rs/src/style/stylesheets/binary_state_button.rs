use super::super::Style;
use iced::*;
use pipe_trait::*;

#[derive(Debug, Clone)]
pub struct BinaryStateButton {
    pub style: Style,
    pub activated: bool,
}

impl button::StyleSheet for BinaryStateButton {
    fn active(&self) -> button::Style {
        let BinaryStateButton { style, activated } = self;

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
