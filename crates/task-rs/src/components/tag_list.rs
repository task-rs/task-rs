use super::super::{data::TagMapIndex, style, utils::Callable};
use super::CheckboxButton;
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Controls(pub BTreeMap<TagMapIndex, button::State>);

pub struct TagList<'a, Theme, GetContent, GetMessage, GetActivated> {
    pub(crate) controls: &'a mut Controls,
    pub get_content: GetContent,
    pub get_message: GetMessage,
    pub get_activated: GetActivated,
    pub theme: Theme,
}

impl<'a, Theme, Message, GetContent, GetMessage, GetActivated> Into<Element<'a, Message>>
    for TagList<'a, Theme, GetContent, GetMessage, GetActivated>
where
    Message: Clone + 'a,
    Theme: style::Theme + Copy,
    GetContent: Callable<Input = TagMapIndex, Output = Element<'a, Message>> + Clone,
    GetMessage: Callable<Input = TagMapIndex, Output = Message> + Clone,
    GetActivated: Callable<Input = TagMapIndex, Output = bool> + Clone,
{
    fn into(self) -> Element<'a, Message> {
        let TagList {
            controls,
            get_activated,
            get_message,
            get_content,
            theme,
        } = self;

        let mut button_list = Column::new();

        for (index, state) in controls.0.iter_mut() {
            let index = *index;
            let activated = get_activated.clone().call(index);
            let button: Button<'a, Message> = CheckboxButton {
                checked: activated,
                content: get_content.clone().call(index),
                state,
            }
            .pipe(Into::<Button<'a, Message>>::into)
            .on_press(get_message.clone().call(index))
            .style(style::BinaryStateButton {
                style: theme.style(),
                activated,
            });
            button_list = button_list.push(button);
        }

        button_list.into()
    }
}

#[derive(Debug, Copy, Clone)]
struct GetStyle<GetActivated, Theme> {
    get_activated: GetActivated,
    theme: Theme,
}
impl<'a, GetActivated, Theme> Callable for GetStyle<GetActivated, Theme>
where
    GetActivated: Callable<Input = TagMapIndex, Output = bool>,
    Theme: style::Theme,
{
    type Input = TagMapIndex;
    type Output = style::BinaryStateButton;
    fn call(self, x: Self::Input) -> Self::Output {
        style::BinaryStateButton {
            activated: self.get_activated.call(x),
            style: self.theme.style(),
        }
    }
}
