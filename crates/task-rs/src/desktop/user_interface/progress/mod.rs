use super::super::super::default_enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Progress {
    Operational,
    Wait,
}

default_enum!(Progress::Operational);
