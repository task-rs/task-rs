pub mod dark;
pub mod light;
pub mod stylesheets;

pub use dark::DARK;
pub use light::LIGHT;
pub use stylesheets::*;

use iced::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Style {
    pub bg: Color,
    pub text: Color,
    pub activated_button_bg: Color,
    pub inactivated_button_bg: Color,
    pub activated_button_text: Color,
    pub inactivated_button_text: Color,
}
