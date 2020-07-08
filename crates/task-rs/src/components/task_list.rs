use super::super::data::{Status, Task};
use super::{Main, Refresh, TaskItem, TaskItemMessage};
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

impl<'a> Refresh<'a> for TaskList {
    fn refresh(main: &'a mut Main) -> Self {
        let mut items = Vec::new();

        fn extend(target: &mut Vec<TaskItem>, tasks: &[Task], address_prefix: &[usize]) {
            for (index, task) in tasks.iter().enumerate() {
                let prefix = || [address_prefix, &[index]].concat();
                target.push(TaskItem::from_task_ref(prefix(), task));
                extend(target, &task.sub, &prefix());
            }
        }

        extend(&mut items, &main.data.tasks, &[]);

        TaskList(items)
    }
}
