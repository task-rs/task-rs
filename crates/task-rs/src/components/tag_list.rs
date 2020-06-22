use super::super::{data::TagId, style, utils::Callable};
use super::{button_list, ButtonList};
use iced::*;
use pipe_trait::*;

pub type Controls = button_list::Controls<TagId>;

pub struct TagList<'a, Theme, Message, GetContent, GetMessage, GetActivated>
where
    Theme: style::Theme,
    GetContent: Callable<Input = &'a TagId, Output = Element<'a, Message>>,
    GetMessage: Callable<Input = &'a TagId, Output = Message>,
    GetActivated: Callable<Input = &'a TagId, Output = bool>,
{
    pub(crate) controls: &'a mut Controls,
    pub get_content: GetContent,
    pub get_message: GetMessage,
    pub get_activated: GetActivated,
    pub theme: Theme,
}

impl<'a, Theme, Message, GetContent, GetMessage, GetActivated> Into<Element<'a, Message>>
    for TagList<'a, Theme, Message, GetContent, GetMessage, GetActivated>
where
    Message: Clone + 'a,
    Theme: style::Theme + Copy,
    GetContent: Callable<Input = &'a TagId, Output = Element<'a, Message>> + Clone,
    GetMessage: Callable<Input = &'a TagId, Output = Message> + Clone,
    GetActivated: Callable<Input = &'a TagId, Output = bool> + Clone,
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
            get_style: GetStyle {
                get_activated,
                theme,
            },
        }
        .pipe(Into::<Column<'a, Message>>::into)
        .into()
    }
}

#[derive(Debug, Copy, Clone)]
struct GetStyle<GetActivated, Theme> {
    get_activated: GetActivated,
    theme: Theme,
}
impl<'a, GetActivated, Theme> Callable for GetStyle<GetActivated, Theme>
where
    GetActivated: Callable<Input = &'a TagId, Output = bool>,
    Theme: style::Theme,
{
    type Input = &'a TagId;
    type Output = style::BinaryStateButton;
    fn call(self, x: Self::Input) -> Self::Output {
        style::BinaryStateButton {
            activated: self.get_activated.call(x),
            style: self.theme.style(),
        }
    }
}
