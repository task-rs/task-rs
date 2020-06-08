pub mod status;
pub mod tag;
pub mod task;

pub use status::Status;
pub use tag::{Data as TagData, Id as TagId};
pub use task::Task;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Data {
    pub manifest_version: String,
    pub tasks: Vec<Task>,
    pub tags: BTreeMap<TagId, TagData>,
}
