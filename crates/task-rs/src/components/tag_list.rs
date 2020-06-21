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
    pub theme: Theme,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for TagList<'a, Theme, Message>
where
    Message: Clone + 'a,
    Theme: style::Theme,
{
    fn into(self) -> Element<'a, Message> {
        ButtonList {
            controls: self.controls,
            get_content: self.get_content,
            get_message: self.get_message,
            theme: self.theme,
        }
        .pipe(Into::<Column<'a, Message>>::into)
        .into()
    }
}
