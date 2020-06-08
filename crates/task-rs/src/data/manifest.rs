use super::{Tag, Task};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    pub manifest_version: String,
    pub tasks: Vec<Task>,
    pub tags: Vec<Tag>,
}
