use super::super::super::components::task_status_filter;
use super::super::{
    model::{details::TaskStatusFilter, Controls, Theme},
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
            .push(task_status_filter::Button {
                state: &mut controls.task_state_filter_all,
                label: "All",
                represented_value: TaskStatusFilter::All,
                message: Message::SetTaskStatusFilter(TaskStatusFilter::All),
                theme,
                actual_value,
            })
            .push(task_status_filter::Button {
                state: &mut controls.task_state_filter_active,
                label: "Active",
                represented_value: TaskStatusFilter::ActiveOnly,
                message: Message::SetTaskStatusFilter(TaskStatusFilter::ActiveOnly),
                theme,
                actual_value,
            })
            .push(task_status_filter::Button {
                state: &mut controls.task_state_filter_completed,
                label: "Completed",
                represented_value: TaskStatusFilter::CompletedOnly,
                message: Message::SetTaskStatusFilter(TaskStatusFilter::CompletedOnly),
                theme,
                actual_value,
            })
            .into()
    }
}
