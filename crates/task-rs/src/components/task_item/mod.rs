pub mod message;
pub mod status_accumulation;

pub use message::Message;
pub use status_accumulation::StatusAccumulation;

use super::super::data::Status;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub task_address: Rc<Vec<usize>>,
    pub task_status: Status,
    pub task_summary: String,
    pub task_status_accumulation: StatusAccumulation,
}

mod from_task_ref;
mod view;
