use super::super::default_enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Uncompleted,
    Completed,
}

default_enum!(Status::Uncompleted);
