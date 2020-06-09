use super::super::super::super::{data::TagId, default_enum};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Serialize, Deserialize)]
pub enum Tasks {
    All,
    SingleTag(TagId),
    MultipleTags(BTreeSet<TagId>),
}

default_enum!(Tasks::All);
