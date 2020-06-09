use super::super::super::super::data::TagId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub enum Tasks {
    All,
    SingleTag(TagId),
    MultipleTags(BTreeSet<TagId>),
}
