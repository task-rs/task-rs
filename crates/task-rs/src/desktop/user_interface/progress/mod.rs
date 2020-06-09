use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Progress {
    Operational,
    Wait,
}

impl Default for Progress {
    fn default() -> Self {
        Progress::Operational
    }
}
