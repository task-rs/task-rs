use super::super::{
    data::TagMapIndex,
    sizes::sidebar::*,
    style::{self, Theme},
    utils::Callable,
};
use super::IndentedButton;
use iced::*;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Controls(pub BTreeMap<TagMapIndex, button::State>);

pub trait TagListParams<'a> {
    type Message: Clone + 'a;
    type Theme: style::Theme + Copy;
    type GetContent: Callable<Input = TagMapIndex, Output = Element<'a, Self::Message>> + Clone;
    type GetMessage: Callable<Input = TagMapIndex, Output = Self::Message> + Clone;
    type GetActivated: Callable<Input = TagMapIndex, Output = bool> + Clone;
}

pub struct TagList<'a, Params: TagListParams<'a>> {
    pub controls: &'a mut Controls,
    pub button_prefix: &'a str,
    pub get_content: Params::GetContent,
    pub get_message: Params::GetMessage,
    pub get_activated: Params::GetActivated,
    pub theme: Params::Theme,
}

impl<'a, Params: TagListParams<'a>> Into<Element<'a, Params::Message>> for TagList<'a, Params> {
    fn into(self) -> Element<'a, Params::Message> {
        let TagList {
            controls,
            button_prefix,
            get_activated,
            get_message,
            get_content,
            theme,
        } = self;

        let mut button_list = Column::new();

        for (index, state) in controls.0.iter_mut() {
            let index = *index;
            let activated = get_activated.clone().call(index);
            let button: Button<'a, Params::Message> = IndentedButton {
                prefix: if activated { button_prefix } else { "" },
                content: get_content.clone().call(index),
                state,
            }
            .into_button()
            .width(Length::Units(SIDEBAR_LENGTH))
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
