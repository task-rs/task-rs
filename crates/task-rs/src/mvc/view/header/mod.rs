use super::super::{
    model::{details::TaskStatusFilter, Controls},
    Message,
};
use iced::*;

pub fn create(controls: &mut Controls) -> Element<'_, Message> {
    Row::new()
        .push(Text::new("TaskRs").size(40))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_all,
            "all",
            TaskStatusFilter::All,
        ))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_active,
            "active",
            TaskStatusFilter::ActiveOnly,
        ))
        .push(task_status_filter::create_button(
            &mut controls.task_state_filter_completed,
            "completed",
            TaskStatusFilter::CompletedOnly,
        ))
        .into()
}

mod task_status_filter;
