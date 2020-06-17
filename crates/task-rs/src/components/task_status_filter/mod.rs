pub mod button;
pub mod controls;

pub use button::{Button, StyleSheet, Value};
pub use controls::Controls;

use super::theme;
use iced::*;

pub struct TaskStatusFilter<'ctrl, Theme, Message>
where
    Theme: theme::Theme,
{
    pub(crate) controls: &'ctrl mut Controls,
    pub actual_value: Value,
    pub theme: Theme,
    pub get_message: fn(Value) -> Message,
}

impl<'ctrl, Theme, Message> Into<Element<'ctrl, Message>>
    for TaskStatusFilter<'ctrl, Theme, Message>
where
    Theme: theme::Theme + Copy,
    Message: Clone + 'ctrl,
{
    fn into(self) -> Element<'ctrl, Message> {
        let TaskStatusFilter {
            controls,
            actual_value,
            theme,
            get_message,
        } = self;

        Row::new()
            .push(Button {
                state: &mut controls.task_state_filter_all,
                label: "All",
                represented_value: Value::All,
                message: get_message(Value::All),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.task_state_filter_active,
                label: "Active",
                represented_value: Value::ActiveOnly,
                message: get_message(Value::ActiveOnly),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.task_state_filter_completed,
                label: "Completed",
                represented_value: Value::CompletedOnly,
                message: get_message(Value::CompletedOnly),
                theme,
                actual_value,
            })
            .into()
    }
}
