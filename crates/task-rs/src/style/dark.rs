use super::Style;
use iced::Color;

pub const DARK: Style = Style {
    bg: Color::BLACK,
    text: Color::WHITE,
    activated_button_bg: Color::from_rgb(0.0, 0.0, 1.0),
    activated_button_text: Color::BLACK,
    inactivated_button_bg: Color::TRANSPARENT,
    inactivated_button_text: Color::WHITE,
};
