use super::super::super::{
    model::{details::TaskStatusFilter, Theme},
    Message,
};
use super::super::stylesheets;
use iced::*;

pub struct Button<'state, 'label, 'theme> {
    pub state: &'state mut button::State,
    pub label: &'label str,
    pub represented_value: TaskStatusFilter,
    pub actual_value: TaskStatusFilter,
    pub theme: &'theme Theme,
}

impl<'state, 'label, 'theme> Into<Element<'state, Message>> for Button<'state, 'label, 'theme> {
    fn into(self) -> Element<'state, Message> {
        let Button {
            state,
            label,
            represented_value,
            actual_value,
            theme,
        } = self;

        iced::Button::new(state, Text::new(label))
            .on_press(Message::SetTaskStatusFilter(represented_value))
            .style(stylesheets::TaskStatusFilter {
                activated: represented_value == actual_value,
                style: theme.style(),
            })
            .into()
    }
}
