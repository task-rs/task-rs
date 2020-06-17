use super::super::{
    model::{details::TaskStatusFilter, Controls, Theme},
    Message,
};
use iced::*;

pub(crate) fn create<'a>(
    controls: &'a mut Controls,
    actual_value: TaskStatusFilter,
    theme: &Theme,
) -> Element<'a, Message> {
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

mod task_status_filter;
