pub mod button;
pub mod controls;

pub use button::{Button, Value};
pub use controls::Controls;

use super::super::style;
use iced::*;

pub struct TaskStatusFilter<'a, Theme> {
    pub controls: &'a mut Controls,
    pub actual_value: Value,
    pub theme: Theme,
}

impl<'a, Theme> TaskStatusFilter<'a, Theme>
where
    Theme: style::Theme + Copy,
{
    pub fn view(self) -> Element<'a, Message> {
        let TaskStatusFilter {
            controls,
            actual_value,
            theme,
        } = self;

        Row::new()
            .push(Button {
                state: &mut controls.all,
                label: "All",
                represented_value: Value::All,
                message: Message(Value::All),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.active,
                label: "Active",
                represented_value: Value::ActiveOnly,
                message: Message(Value::ActiveOnly),
                theme,
                actual_value,
            })
            .push(Button {
                state: &mut controls.completed,
                label: "Completed",
                represented_value: Value::CompletedOnly,
                message: Message(Value::CompletedOnly),
                theme,
                actual_value,
            })
            .into()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Message(pub Value);
