use super::super::super::{
    data::{TagId, Task},
    utils::merge_sets_with_minimal_cloning,
};
use smart_default::SmartDefault;
use std::collections::BTreeSet;

#[derive(Debug, SmartDefault, Clone, Eq, PartialEq)]
pub enum TagAccumulation {
    Satisfied,
    #[default]
    Unsatisfied(BTreeSet<TagId>),
}

impl TagAccumulation {
    pub fn join(&mut self, other: &Self) {
        if let TagAccumulation::Unsatisfied(a) = self {
            match other {
                TagAccumulation::Satisfied => {
                    *self = TagAccumulation::Satisfied;
                }
                TagAccumulation::Unsatisfied(b) => {
                    merge_sets_with_minimal_cloning(a, b);
                }
            }
        }
    }

    pub fn join_tags(&mut self, tags: &BTreeSet<TagId>) {
        if let TagAccumulation::Unsatisfied(existing_tags) = self {
            merge_sets_with_minimal_cloning(existing_tags, tags);
        }
    }

    pub fn join_task(&mut self, task: &Task) {
        self.join_tags(&task.tags);
    }

    pub fn compare(&mut self, tags: &BTreeSet<TagId>) {
        if let TagAccumulation::Unsatisfied(existing_tags) = self {
            if existing_tags.is_superset(tags) {
                *self = TagAccumulation::Satisfied;
            }
        }
    }

    pub fn is_satisfied(&self) -> bool {
        self == &TagAccumulation::Satisfied
    }
}
