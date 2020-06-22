use super::super::super::super::data::TagId;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::collections::BTreeSet;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tasks {
    pub filter_method: FilterMethod,
    pub single_tag: Option<TagId>,
    pub multiple_tags: BTreeSet<TagId>,
}

#[derive(Debug, SmartDefault, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum FilterMethod {
    #[default]
    All,
    SingleTag,
    MultipleTags,
}
