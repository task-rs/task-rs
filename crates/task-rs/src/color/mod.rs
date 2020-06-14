pub mod dark;
pub mod light;

pub use dark::DARK;
pub use light::LIGHT;

use iced::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct ColorScheme {
    pub bg: Color,
    pub text: Color,
}
