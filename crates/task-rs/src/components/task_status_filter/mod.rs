pub mod button;
pub mod controls;

pub use button::{Button, Value};
pub use controls::Controls;

use super::super::style;
use iced::*;

pub struct TaskStatusFilter<'a, Theme, Message> {
    pub controls: &'a mut Controls,
    pub actual_value: Value,
    pub theme: Theme,
    pub get_message: fn(Value) -> Message,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for TaskStatusFilter<'a, Theme, Message>
where
    Theme: style::Theme + Copy,
    Message: Clone + 'a,
{
    fn into(self) -> Element<'a, Message> {
        let TaskStatusFilter {
            controls,
            actual_value,
            theme,
            get_message,
        } = self;

        Row::new()
            .push(Button {
                state: &mut controls.all,
                label: "All",
                represented_value: Value::All,
                message: get_message(Value::All),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.active,
                label: "Active",
                represented_value: Value::ActiveOnly,
                message: get_message(Value::ActiveOnly),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.completed,
                label: "Completed",
                represented_value: Value::CompletedOnly,
                message: get_message(Value::CompletedOnly),
                theme,
                actual_value,
            })
            .into()
    }
}
