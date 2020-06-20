use super::super::Style;
use iced::*;
use pipe_trait::*;

pub struct SingleStateButton(pub Style);
impl button::StyleSheet for SingleStateButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::TRANSPARENT.pipe(Background::Color).pipe(Some),
            text_color: self.0.text,
            ..button::Style::default()
        }
    }
}
