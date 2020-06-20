use super::model::{details::TaskStatusFilter, view::tasks::FilterMethod};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    SetTaskStatusFilter(TaskStatusFilter),
    SetDarkMode(bool),
    SetTaskFilterMethod(FilterMethod),
}
