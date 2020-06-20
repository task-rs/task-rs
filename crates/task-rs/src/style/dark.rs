use super::Style;
use iced::Color;

pub const DARK: Style = Style {
    bg: Color::BLACK,
    text: Color::WHITE,
    task_status_filter_activated_bg: Color::from_rgb(0.0, 0.0, 1.0),
    task_status_filter_activated_text: Color::BLACK,
    task_status_filter_inactivated_bg: Color::TRANSPARENT,
    task_status_filter_inactivated_text: Color::WHITE,
    tag_filter_method_activated_bg: Color::from_rgb(0.0, 0.0, 1.0),
    tag_filter_method_activated_text: Color::BLACK,
    tag_filter_method_inactivated_bg: Color::TRANSPARENT,
    tag_filter_method_inactivated_text: Color::WHITE,
};
