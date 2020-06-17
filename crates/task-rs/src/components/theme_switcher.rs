use super::theme;
use iced::*;

pub struct ThemeSwitcher<Theme, Message>
where
    Theme: theme::Theme + Copy,
{
    pub dark_mode: bool,
    pub theme: Theme,
    pub get_message: fn(bool) -> Message,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for ThemeSwitcher<Theme, Message>
where
    Theme: theme::Theme + Copy,
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let ThemeSwitcher {
            dark_mode,
            get_message,
            ..
        } = self;

        Checkbox::new(dark_mode, "Dark Mode", get_message).into()
    }
}
