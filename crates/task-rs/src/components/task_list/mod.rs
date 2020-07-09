use super::super::data::Status;
use super::{Main, Refresh, TaskItem, TaskItemMessage};
use iced::*;

#[derive(Debug, Default, Clone)]
pub struct TaskList {
    pub tasks: Vec<TaskItem>,
}

impl TaskList {
    pub fn view(&self) -> Element<'_, Message> {
        let TaskList { tasks } = self;

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

impl<'a> Refresh<'a> for TaskList {
    fn refresh(main: &'a mut Main) -> Self {
        let mut tasks = Vec::new();
        utils::extend_task_item_list(&mut tasks, &main.data.tasks, &[], Default::default());
        TaskList { tasks }
    }
}

mod utils;
