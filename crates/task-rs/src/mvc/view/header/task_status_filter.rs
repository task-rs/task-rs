use super::super::super::{
    model::{details::TaskStatusFilter, Theme},
    Message,
};
use super::super::stylesheets;
use iced::*;

pub fn create_button<'a>(
    state: &'a mut button::State,
    label: &str,
    represented_value: TaskStatusFilter,
    actual_value: TaskStatusFilter,
    theme: &Theme,
) -> Element<'a, Message> {
    Button::new(state, Text::new(label))
        .on_press(Message::SetTaskStatusFilter(represented_value))
        .style(stylesheets::TaskStatusFilter {
            activated: represented_value == actual_value,
            style: theme.style(),
        })
        .into()
}
