use super::Style;
use iced::Color;

pub const LIGHT: Style = Style {
    bg: Color::WHITE,
    text: Color::BLACK,
    activated_button_bg: Color::from_rgb(0.0, 0.0, 1.0),
    activated_button_text: Color::WHITE,
    inactivated_button_bg: Color::TRANSPARENT,
    inactivated_button_text: Color::BLACK,
};
