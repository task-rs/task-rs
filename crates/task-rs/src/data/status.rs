use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Uncompleted,
    Completed,
}

impl Default for Status {
    fn default() -> Self {
        Status::Uncompleted
    }
}
