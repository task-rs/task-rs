use super::{Index, IndexedMap};
use bimap::BiBTreeMap;
use std::{collections::BTreeMap, rc::Rc};

impl<Key, Value> From<BTreeMap<Key, Value>> for IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    fn from(source: BTreeMap<Key, Value>) -> Self {
        let mut entries = BTreeMap::new();
        let mut indices = BiBTreeMap::new();
        let counter = Index::from(source.len() as u32);

        for ((key, value), index) in source.into_iter().zip(0u32..) {
            let key = Rc::new(key);
            let index = Index::from(index);
            entries.insert(key.clone(), value);
            indices.insert(key, index);
        }

        IndexedMap {
            entries,
            indices,
            counter,
        }
    }
}

impl<Key, Value> Into<BTreeMap<Key, Value>> for IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    fn into(self) -> BTreeMap<Key, Value> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_ref().clone(), value.clone()))
            .collect()
    }
}
