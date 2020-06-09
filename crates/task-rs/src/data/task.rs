use super::{Status, TagId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Task {
    pub status: Status,
    pub summary: String,
    pub details: String,
    pub sub: Vec<Task>,
    pub tags: BTreeSet<TagId>,
}
