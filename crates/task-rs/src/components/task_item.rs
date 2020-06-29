use super::super::data::{Status, TagId, Task};
use iced::*;
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

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Check => self.task_status = Status::Completed,
            Message::Uncheck => self.task_status = Status::Active,
        }
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        Checkbox::new(
            match self.task_status {
                Status::Active => false,
                Status::Completed => true,
            },
            &self.task_summary,
            |is_checked| {
                if is_checked {
                    Message::Check
                } else {
                    Message::Uncheck
                }
            },
        )
        .into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    Check,
    Uncheck,
}
