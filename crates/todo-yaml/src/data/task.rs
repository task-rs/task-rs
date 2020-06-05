use super::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub status: Status,
    pub summary: String,
    pub details: String,
    pub sub: Vec<Task>,
}
