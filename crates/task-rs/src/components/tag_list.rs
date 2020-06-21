use super::super::{data::TagId, style};
use super::{button_list, ButtonList};
use iced::*;
use pipe_trait::*;

pub type Controls = button_list::Controls<TagId>;

pub struct TagList<'a, Theme, Message>
where
    Theme: style::Theme,
{
    pub(crate) controls: &'a mut Controls,
    pub get_content: fn(&TagId) -> Element<'a, Message>,
    pub get_message: fn(&TagId) -> Message,
    pub get_activated: fn(&TagId) -> bool,
    pub theme: Theme,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for TagList<'a, Theme, Message>
where
    Message: Clone + 'a,
    Theme: style::Theme + Copy,
{
    fn into(self) -> Element<'a, Message> {
        let TagList {
            controls,
            get_activated,
            get_message,
            get_content,
            theme,
        } = self;

        ButtonList {
            controls,
            get_content,
            get_message,
            get_style: |id| style::BinaryStateButton {
                activated: get_activated(id),
                style: theme.style(),
            },
        }
        .pipe(Into::<Column<'a, Message>>::into)
        .into()
    }
}
