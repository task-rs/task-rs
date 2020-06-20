use super::super::data::TagId;
use super::model::{details::TaskStatusFilter, view::tasks::FilterMethod};

#[derive(Debug, Clone)]
pub enum Message {
    SetTaskStatusFilter(TaskStatusFilter),
    SetDarkMode(bool),
    SetTaskFilterMethod(FilterMethod),
    FilterTasksBySingleTag(TagId),
}
