pub mod status;
pub mod tag;
pub mod task;

pub use status::Status;
pub use tag::Tag;
pub use task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Data {
    pub manifest_version: String,
    pub tasks: Vec<Task>,
    pub tags: Vec<Tag>,
}
