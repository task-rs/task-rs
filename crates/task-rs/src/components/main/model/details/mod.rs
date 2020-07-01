use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Details {
    pub task_status_filter: TaskStatusFilter,
}

pub use super::super::super::task_status_filter::Value as TaskStatusFilter;
