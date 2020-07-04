use super::super::super::style;
use super::{Controls, Message};
use iced::*;

pub struct ThemeSwitcher<'a, Theme> {
    pub dark_mode: bool,
    pub theme: Theme,
    pub controls: &'a mut Controls,
}

impl<'a, Theme> ThemeSwitcher<'a, Theme>
where
    Theme: style::Theme + Copy,
{
    pub fn view(self) -> Element<'a, Message> {
        let ThemeSwitcher {
            dark_mode,
            controls,
            theme,
        } = self;

        let label = if dark_mode { "Light Mode" } else { "Dark Mode" };

        Button::new(&mut controls.0, Text::new(label))
            .on_press(Message(!dark_mode))
            .style(style::SingleStateButton(theme.style()))
            .into()
    }
}
