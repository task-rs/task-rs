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
            .push(task_status_filter::create_button(
                &mut controls.task_state_filter_all,
                "All",
                TaskStatusFilter::All,
                actual_value,
                theme,
            ))
            .push(task_status_filter::create_button(
                &mut controls.task_state_filter_active,
                "Active",
                TaskStatusFilter::ActiveOnly,
                actual_value,
                theme,
            ))
            .push(task_status_filter::create_button(
                &mut controls.task_state_filter_completed,
                "Completed",
                TaskStatusFilter::CompletedOnly,
                actual_value,
                theme,
            ))
            .into()
    }
}

mod task_status_filter;
