use super::super::super::style;
use super::super::theme;
use iced::*;
use pipe_trait::*;
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

pub struct Button<'state, Label, Theme, Message>
where
    Label: Into<String>,
    Theme: theme::Theme,
    Message: Clone,
{
    pub state: &'state mut button::State,
    pub label: Label,
    pub represented_value: Value,
    pub actual_value: Value,
    pub theme: Theme,
    pub message: Message,
}

impl<'state, Label, Theme, Message> Into<Element<'state, Message>>
    for Button<'state, Label, Theme, Message>
where
    Label: Into<String>,
    Theme: theme::Theme,
    Message: Clone + 'state,
{
    fn into(self) -> Element<'state, Message> {
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
            .style(StyleSheet {
                activated: represented_value == actual_value,
                style: theme.style(),
            })
            .into()
    }
}

pub struct StyleSheet {
    pub style: style::Style,
    pub activated: bool,
}
impl button::StyleSheet for StyleSheet {
    fn active(&self) -> button::Style {
        let StyleSheet { style, activated } = self;

        button::Style {
            background: if *activated {
                style.task_status_filter_activated_bg
            } else {
                style.task_status_filter_inactivated_bg
            }
            .pipe(Background::Color)
            .pipe(Some),
            text_color: if *activated {
                style.task_status_filter_activated_text
            } else {
                style.task_status_filter_inactivated_text
            },
            ..button::Style::default()
        }
    }
}
