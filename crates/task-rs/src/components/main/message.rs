use super::super::super::data::{Status, TagMapIndex};
use super::{model::view::tasks::FilterMethod, task_status_filter::Value as TaskStatusFilter};

#[derive(Debug, Clone)]
pub enum Message {
    MultipleActions(Vec<Message>),
    Warn(String),
    SetTaskStatusFilter(TaskStatusFilter),
    SetDarkMode(bool),
    SetTaskFilterMethod(FilterMethod),
    FilterTasksBySingleTag(TagMapIndex),
    AddTagToMultipleTags(TagMapIndex),
    RemoveTagFromMultipleTags(TagMapIndex),
    CheckAllOfMultipleTags,
    UncheckAllOfMultipleTags,
    InvertAllOfMultipleTags,
    SetTaskStatus(Vec<usize>, Status),
}
