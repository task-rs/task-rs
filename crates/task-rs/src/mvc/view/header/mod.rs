use super::super::{
    model::{details::TaskStatusFilter, Controls},
    Message,
};
use iced::*;

pub(crate) fn create(controls: &mut Controls) -> Element<'_, Message> {
    Row::new()
        .push(Text::new("TaskRs").size(40))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_all,
            "All",
            TaskStatusFilter::All,
        ))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_active,
            "Active",
            TaskStatusFilter::ActiveOnly,
        ))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_completed,
            "Completed",
            TaskStatusFilter::CompletedOnly,
        ))
        .into()
}

mod task_status_filter;
