use super::super::super::style;
use super::super::theme;
use super::Controls;
use iced::*;
use pipe_trait::*;

pub struct ThemeSwitcher<'a, Theme, Message>
where
    Theme: theme::Theme + Copy,
{
    pub dark_mode: bool,
    pub theme: Theme,
    pub get_message: fn(bool) -> Message,
    pub controls: &'a mut Controls,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for ThemeSwitcher<'a, Theme, Message>
where
    Theme: theme::Theme + Copy,
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let ThemeSwitcher {
            dark_mode,
            get_message,
            controls,
            theme,
        } = self;

        let label = if dark_mode { "Light Mode" } else { "Dark Mode" };

        Button::new(&mut controls.0, Text::new(label))
            .on_press(get_message(!dark_mode))
            .style(StyleSheet(theme.style()))
            .into()
    }
}

struct StyleSheet(pub style::Style);
impl button::StyleSheet for StyleSheet {
    fn active(&self) -> button::Style {
        button::Style {
            background: Color::TRANSPARENT.pipe(Background::Color).pipe(Some),
            text_color: self.0.text,
            ..button::Style::default()
        }
    }
}
