use super::super::{data::Status, utils::Callable};
use super::{TaskItem, TaskItemMessage};
use iced::*;

pub struct TaskList<'a, GetMessage>
where
    GetMessage: Callable<Input = (Vec<usize>, Status)> + Copy + 'static,
{
    pub tasks: &'a Vec<TaskItem>,
    pub get_message: GetMessage,
}

impl<'a, Message, GetMessage> Into<Element<'a, Message>> for TaskList<'a, GetMessage>
where
    Message: Clone + 'static,
    GetMessage: Callable<Input = (Vec<usize>, Status), Output = Message> + Copy + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let TaskList { tasks, get_message } = self;

        let mut column = Column::new();

        for (index, item) in tasks.iter().enumerate() {
            let element = item.view().map(move |message| match message {
                TaskItemMessage::SetStatus(status) => get_message.call((vec![index], status)),
            });
            column = column.push(element);
        }

        column.into()
    }
}
