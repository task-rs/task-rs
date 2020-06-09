use super::super::super::super::data::TagId;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::BTreeSet;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub enum Tasks {
    #[default]
    All,
    SingleTag(TagId),
    MultipleTags(BTreeSet<TagId>),
}
