pub mod message;
pub mod status_accumulation;
pub mod tag_accumulation;

pub use message::Message;
pub use status_accumulation::StatusAccumulation;
pub use tag_accumulation::TagAccumulation;

use super::super::data::Status;
use std::rc::Rc;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TaskItem {
    pub address: Rc<Vec<usize>>,
    pub status: Status,
    pub summary: String,
    pub status_accumulation: StatusAccumulation,
    pub tag_accumulation: TagAccumulation,
}

mod from_task_ref;
mod view;
