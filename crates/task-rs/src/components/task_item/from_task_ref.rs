use super::super::super::data::Task;
use super::{StatusAccumulation, TaskItem};
use std::rc::Rc;

impl TaskItem {
    pub fn from_task_ref(
        task_address: Vec<usize>,
        task: &Task,
        status_accumulation: StatusAccumulation,
    ) -> Self {
        TaskItem {
            address: Rc::new(task_address),
            status: task.status,
            summary: task.summary.clone(),
            status_accumulation: status_accumulation.join_task(task),
        }
    }
}
