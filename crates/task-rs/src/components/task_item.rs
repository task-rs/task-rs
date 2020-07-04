use super::super::data::{Status, Task};
use iced::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub task_address: Vec<usize>,
    pub task_status: Status,
    pub task_summary: String,
}

impl TaskItem {
    pub fn from_task_ref(task_address: Vec<usize>, task: &Task) -> Self {
        TaskItem {
            task_address,
            task_status: task.status,
            task_summary: task.summary.clone(),
        }
    }

    pub fn view(self) -> Element<'static, Message> {
        Checkbox::new(
            match self.task_status {
                Status::Active => false,
                Status::Completed => true,
            },
            self.task_summary.clone(),
            move |is_checked| {
                Message::SetStatus(
                    self.task_address.clone(),
                    if is_checked {
                        Status::Completed
                    } else {
                        Status::Active
                    },
                )
            },
        )
        .into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    SetStatus(Vec<usize>, Status),
}
