use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Details {
    pub task_status_filter: TaskStatusFilter,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStatusFilter {
    All,
    #[default]
    ActiveOnly,
    CompletedOnly,
}
