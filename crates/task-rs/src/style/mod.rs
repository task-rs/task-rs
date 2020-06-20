pub mod dark;
pub mod light;

pub use dark::DARK;
pub use light::LIGHT;

use iced::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    pub bg: Color,
    pub text: Color,
    pub task_status_filter_activated_bg: Color,
    pub task_status_filter_inactivated_bg: Color,
    pub task_status_filter_activated_text: Color,
    pub task_status_filter_inactivated_text: Color,
    pub tag_filter_method_activated_bg: Color,
    pub tag_filter_method_inactivated_bg: Color,
    pub tag_filter_method_activated_text: Color,
    pub tag_filter_method_inactivated_text: Color,
}
