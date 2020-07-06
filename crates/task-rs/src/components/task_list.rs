use super::super::data::Status;
use super::{TaskItem, TaskItemMessage};
use iced::*;

#[derive(Debug, Default, Clone)]
pub struct TaskList(pub Vec<TaskItem>);

impl TaskList {
    pub fn view(&self) -> Element<'_, Message> {
        let TaskList(tasks) = self;

        let mut column = Column::new();

        for item in tasks {
            let element = item.clone().view().map(move |message| match message {
                TaskItemMessage::SetStatus(address, status) => Message::SetStatus(address, status),
            });
            column = column.push(element);
        }

        column.into()
    }
}

pub enum Message {
    SetStatus(Vec<usize>, Status),
}
