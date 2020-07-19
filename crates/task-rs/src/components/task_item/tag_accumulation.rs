use super::super::super::data::{TagId, Task};
use smart_default::SmartDefault;
use std::collections::BTreeSet;

#[derive(Debug, SmartDefault, Clone, Copy, Eq, PartialEq)]
pub struct TagAccumulation {
    #[default(false)]
    pub satisfaction: bool,
}

impl TagAccumulation {
    pub fn from_bool(satisfaction: bool) -> Self {
        TagAccumulation { satisfaction }
    }

    pub fn calculate(task: &Task, tags: &BTreeSet<TagId>) -> Self {
        Self::from_bool(task.tags.is_superset(tags))
    }
}
