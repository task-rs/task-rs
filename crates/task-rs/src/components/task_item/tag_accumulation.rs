use super::super::super::data::TagId;
use smart_default::SmartDefault;
use std::collections::BTreeSet;

#[derive(Debug, SmartDefault, Clone, Eq, PartialEq)]
pub struct TagAccumulation {
    pub tags: BTreeSet<TagId>,
    #[default(false)]
    pub satisfy: bool,
}
