use super::super::data::Status;
use super::{TaskItem, TaskItemMessage};
use iced::*;

#[derive(Debug, Default, Clone)]
pub struct TaskList(pub Vec<TaskItem>);

impl TaskList {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SetStatus(address, status) => {
                for task in self.0.iter_mut() {
                    if task.task_address == address {
                        task.task_status = status;
                        break;
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let TaskList(tasks) = self;

        let mut column = Column::new();

        for (index, item) in tasks.iter().enumerate() {
            let element = item.view().map(move |message| match message {
                TaskItemMessage::SetStatus(status) => Message::SetStatus(vec![index], status),
            });
            column = column.push(element);
        }

        column.into()
    }
}

pub enum Message {
    SetStatus(Vec<usize>, Status),
}
