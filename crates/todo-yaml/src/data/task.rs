use super::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Task {
    pub status: Status,
    pub summary: String,
    pub details: String,
    pub sub: Vec<Task>,
}
