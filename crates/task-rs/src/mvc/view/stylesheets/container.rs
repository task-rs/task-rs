use super::super::super::super::style;
use iced::{container, Background};

pub struct StyleSheet(pub style::Style);
impl container::StyleSheet for StyleSheet {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.0.bg)),
            text_color: Some(self.0.text),
            ..container::Style::default()
        }
    }
}
