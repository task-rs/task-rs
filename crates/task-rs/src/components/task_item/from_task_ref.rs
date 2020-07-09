use super::super::super::data::{Status, Task};
use super::{StatusAccumulation, TaskItem};
use std::rc::Rc;

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
}
