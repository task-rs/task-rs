use super::super::data::{Status, TagMapIndex};
use super::model::{details::TaskStatusFilter, view::tasks::FilterMethod};

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
