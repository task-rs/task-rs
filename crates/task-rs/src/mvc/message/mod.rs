use super::super::data::TagMapIndex;
use super::model::{details::TaskStatusFilter, view::tasks::FilterMethod};

// TODO: Add 'Copy'
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
}
