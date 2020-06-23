pub mod methods;

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(from = "BTreeMap<Key, Value>", into = "BTreeMap<Key, Value>")]
pub struct IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    key_value: BTreeMap<Rc<Key>, Value>,
    index_key: BTreeMap<u32, Rc<Key>>,
    key_index: BTreeMap<Rc<Key>, u32>,
    counter: u32,
}

mod convert;
mod eq;

#[cfg(test)]
mod tests;
