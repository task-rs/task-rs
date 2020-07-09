use super::super::super::super::data::{Status, Task};
use super::find_task::find_task;

pub fn set_task_status(target: &mut Vec<Task>, address: &[usize], status: Status) {
    if let Some(task) = find_task(target, address) {
        task.status = status;
    }
}
