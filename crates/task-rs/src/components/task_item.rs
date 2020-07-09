use super::super::{
    data::{Status, Task},
    sizes::main::SUB_TASK_INDENT,
};
use iced::*;
use pipe_trait::*;
use smart_default::SmartDefault;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub task_address: Rc<Vec<usize>>,
    pub task_status: Status,
    pub task_summary: String,
    pub task_status_accumulation: StatusAccumulation,
}

impl TaskItem {
    pub fn from_task_ref(
        task_address: Vec<usize>,
        task: &Task,
        task_status_accumulation: StatusAccumulation,
    ) -> Self {
        TaskItem {
            task_address: Rc::new(task_address),
            task_status: task.status,
            task_summary: task.summary.clone(),
            task_status_accumulation: task_status_accumulation
                .join_all_active(task.status == Status::Active)
                .join_some_completed(task.status == Status::Completed),
        }
    }

    pub fn view(self) -> Element<'static, Message> {
        let indent_size = self
            .task_address
            .as_ref()
            .len()
            .pipe(|x| x as u16 * SUB_TASK_INDENT)
            .pipe(Length::Units);

        let indentation = Text::new("")
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

#[derive(Debug, SmartDefault, Copy, Clone, Eq, PartialEq)]
pub struct StatusAccumulation {
    #[default(true)]
    all_active: bool,
    #[default(false)]
    some_completed: bool,
}

#[test]
fn test_recursive_status_default() {
    assert_eq!(
        StatusAccumulation::default(),
        StatusAccumulation {
            all_active: true,
            some_completed: false,
        }
    )
}

impl StatusAccumulation {
    pub fn join(self, other: Self) -> Self {
        self.join_all_active(other.all_active)
            .join_some_completed(other.some_completed)
    }

    pub fn join_all_active(self, other: bool) -> Self {
        StatusAccumulation {
            all_active: self.all_active && other,
            ..self
        }
    }

    pub fn join_some_completed(self, other: bool) -> Self {
        StatusAccumulation {
            some_completed: self.some_completed && other,
            ..self
        }
    }
}
