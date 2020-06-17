use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Details {
    pub task_status_filter: TaskStatusFilter,
}

pub use super::super::super::components::task_status_filter::Value as TaskStatusFilter;
