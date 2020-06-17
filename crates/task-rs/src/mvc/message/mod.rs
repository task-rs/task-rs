use super::model::details::TaskStatusFilter;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    SetTaskStatusFilter(TaskStatusFilter),
    SetDarkMode(bool),
}
