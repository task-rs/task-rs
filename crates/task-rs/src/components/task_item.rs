use super::super::data::{Status, TagId, Task};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub task_address: Vec<usize>,
    pub task_status: Status,
    pub task_summary: String,
    pub task_details: String,
    pub task_tags: BTreeSet<TagId>,
}

impl TaskItem {
    pub fn from_task_ref(task_address: Vec<usize>, task: &Task) -> Self {
        TaskItem {
            task_address,
            task_status: task.status,
            task_summary: task.summary.clone(),
            task_details: task.details.clone(),
            task_tags: task.tags.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Message {
    Check,
    Uncheck,
}
