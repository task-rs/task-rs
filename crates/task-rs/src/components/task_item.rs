use super::super::{
    data::{Status, Task},
    sizes::main::SUB_TASK_INDENT,
};
use iced::*;
use pipe_trait::*;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub task_address: Rc<Vec<usize>>,
    pub task_status: Status,
    pub task_summary: String,
}

impl TaskItem {
    pub fn from_task_ref(task_address: Vec<usize>, task: &Task) -> Self {
        TaskItem {
            task_address: Rc::new(task_address),
            task_status: task.status,
            task_summary: task.summary.clone(),
        }
    }

    pub fn view(self) -> Element<'static, Message> {
        let indent_size = self
            .task_address
            .as_ref()
            .len()
            .pipe(|x| x as u16 * SUB_TASK_INDENT)
            .pipe(Length::Units);

        let indentation = Container::new(Text::new(""))
            .width(indent_size)
            .height(Length::Units(SUB_TASK_INDENT));

        let checkbox = Checkbox::new(
            match self.task_status {
                Status::Active => false,
                Status::Completed => true,
            },
            self.task_summary.clone(),
            move |is_checked| {
                Message::SetStatus(
                    self.task_address.as_ref().clone(),
                    if is_checked {
                        Status::Completed
                    } else {
                        Status::Active
                    },
                )
            },
        );

        Row::new().push(indentation).push(checkbox).into()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    SetStatus(Vec<usize>, Status),
}
