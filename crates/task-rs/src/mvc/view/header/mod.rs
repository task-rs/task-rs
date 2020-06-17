use super::super::super::components::task_status_filter::{self, Controls};
use super::super::{
    model::{details::TaskStatusFilter, Theme},
    Message,
};
use iced::*;

pub(crate) struct Header<'ctrl, 'theme> {
    pub controls: &'ctrl mut Controls,
    pub actual_value: TaskStatusFilter,
    pub theme: &'theme Theme,
}

impl<'ctrl, 'theme> Into<Element<'ctrl, Message>> for Header<'ctrl, 'theme> {
    fn into(self) -> Element<'ctrl, Message> {
        let Header {
            controls,
            actual_value,
            theme,
        } = self;

        Row::new()
            .push(Text::new("TaskRs").size(40))
            .push(task_status_filter::TaskStatusFilter {
                get_message: Message::SetTaskStatusFilter,
                controls,
                actual_value,
                theme,
            })
            .into()
    }
}
