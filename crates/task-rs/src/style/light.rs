use super::Style;
use iced::Color;

pub const LIGHT: Style = Style {
    bg: Color::WHITE,
    text: Color::BLACK,
    task_status_filter_activated_bg: Color::from_rgb(0.0, 0.0, 1.0),
    task_status_filter_activated_text: Color::WHITE,
    task_status_filter_inactivated_bg: Color::TRANSPARENT,
    task_status_filter_inactivated_text: Color::BLACK,
};
