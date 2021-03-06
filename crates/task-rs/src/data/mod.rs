pub mod manifest_version;
pub mod status;
pub mod tag;
pub mod task;

pub use manifest_version::ManifestVersion;
pub use status::Status;
pub use tag::{Data as TagData, Id as TagId, Index as TagMapIndex, Map as TagMap};
pub use task::Task;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Data {
    pub manifest_version: ManifestVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tasks: Vec<Task>,
    #[serde(default, skip_serializing_if = "TagMap::is_empty")]
    pub tags: TagMap,
}
