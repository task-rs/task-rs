use super::super::{
    data::{Status, Task},
    utils::Callable,
};
use iced::*;

pub struct TaskList<'a, GetMessage>
where
    GetMessage: Callable<Input = (Vec<usize>, Status)> + Copy + 'static,
{
    pub tasks: &'a Vec<Task>,
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

        for (task, index) in tasks.iter().zip(0..) {
            let Task {
                status, summary, ..
            } = task;

            let checkbox =
                Checkbox::new(*status == Status::Completed, summary, move |is_checked| {
                    let status = if is_checked {
                        Status::Completed
                    } else {
                        Status::Active
                    };
                    get_message.call((vec![index], status))
                });

            column = column.push(checkbox);
        }

        column.into()
    }
}
