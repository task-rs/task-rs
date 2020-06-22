use super::super::data::TagId;
use super::model::{details::TaskStatusFilter, view::tasks::FilterMethod};

// TODO: Add 'Copy'
#[derive(Debug, Clone)]
pub enum Message {
    MultipleActions(Vec<Message>),
    Warn(String),
    SetTaskStatusFilter(TaskStatusFilter),
    SetDarkMode(bool),
    SetTaskFilterMethod(FilterMethod),
    FilterTasksBySingleTag(TagId), // TODO: This is expensive, try replacing it with reference
    AddTagToMultipleTags(TagId),
    RemoveTagFromMultipleTags(TagId),
}
