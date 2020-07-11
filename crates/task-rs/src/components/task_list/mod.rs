use super::super::data::Status;
use super::{
    task_status_filter::{self, Value::*},
    Main, Refresh, TaskItem, TaskItemMessage,
};
use iced::*;

#[derive(Debug, Default, Clone)]
pub struct TaskList {
    pub tasks: Vec<TaskItem>,
    pub task_status_filter: task_status_filter::Value,
}

impl TaskList {
    pub fn view(&self) -> Element<'_, Message> {
        let TaskList {
            tasks,
            task_status_filter,
        } = self;

        let mut column = Column::new();

        for item in tasks {
            if match task_status_filter {
                All => false,
                ActiveOnly => !item.task_status_accumulation.all_active,
                CompletedOnly => {
                    let acc = item.task_status_accumulation;
                    !acc.some_completed && !acc.contains_completed
                }
            } {
                continue;
            }

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
        TaskList {
            tasks: utils::create_task_item_list(&main.data.tasks),
            task_status_filter: main.ui_state.details.task_status_filter,
        }
    }
}

mod utils;
