use super::super::super::{model::details::TaskStatusFilter, Message};
use iced::*;

pub fn create_button<'a>(
    state: &'a mut button::State,
    label: &str,
    value: TaskStatusFilter,
) -> Element<'a, Message> {
    Button::new(state, Text::new(label))
        .on_press(Message::SetTaskStatusFilter(value))
        .into()
}
