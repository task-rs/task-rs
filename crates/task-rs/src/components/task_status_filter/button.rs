use super::super::super::style;
use iced::*;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Value {
    All,
    #[default]
    ActiveOnly,
    CompletedOnly,
}

pub struct Button<'a, Label, Theme, Message>
where
    Label: Into<String>,
    Theme: style::Theme,
    Message: Clone,
{
    pub state: &'a mut button::State,
    pub label: Label,
    pub represented_value: Value,
    pub actual_value: Value,
    pub theme: Theme,
    pub message: Message,
}

impl<'a, Label, Theme, Message> Into<Element<'a, Message>> for Button<'a, Label, Theme, Message>
where
    Label: Into<String>,
    Theme: style::Theme,
    Message: Clone + 'a,
{
    fn into(self) -> Element<'a, Message> {
        let Button {
            state,
            label,
            represented_value,
            actual_value,
            theme,
            message,
        } = self;

        iced::Button::new(state, Text::new(label))
            .on_press(message)
            .style(style::BinaryStateButton {
                activated: represented_value == actual_value,
                style: theme.style(),
            })
            .into()
    }
}
