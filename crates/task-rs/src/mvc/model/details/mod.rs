use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Details {
    pub task_status_filter: TaskStatusFilter,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TaskStatusFilter {
    #[default(true)]
    pub active: bool,
    #[default(false)]
    pub completed: bool,
}
