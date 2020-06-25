pub mod index;
pub mod methods;

pub use index::Index;
pub use methods::{InsertResult, RemoveResult};

use super::BidirectionalMap;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(from = "BTreeMap<Key, Value>", into = "BTreeMap<Key, Value>")]
pub struct IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    entries: BTreeMap<Rc<Key>, Value>,
    indices: BidirectionalMap<Key, Index>,
    counter: Index,
}

mod convert;
mod eq;

#[cfg(test)]
mod tests;
