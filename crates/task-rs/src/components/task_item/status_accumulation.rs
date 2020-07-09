use super::super::super::data::{Status, Task};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Copy, Clone, Eq, PartialEq)]
pub struct StatusAccumulation {
    #[default(true)]
    pub all_active: bool,
    #[default(false)]
    pub some_completed: bool,
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
            some_completed: self.some_completed || other,
            ..self
        }
    }

    pub fn join_task(self, task: &Task) -> Self {
        self.join_all_active(task.status == Status::Active)
            .join_some_completed(task.status == Status::Completed)
    }
}
