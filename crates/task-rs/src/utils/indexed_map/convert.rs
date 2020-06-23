use super::{Index, IndexedMap};
use std::{collections::BTreeMap, rc::Rc};

impl<Key, Value> From<BTreeMap<Key, Value>> for IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    fn from(source: BTreeMap<Key, Value>) -> Self {
        let mut key_value = BTreeMap::new();
        let mut index_key = BTreeMap::new();
        let mut key_index = BTreeMap::new();
        let counter = Index::from(source.len() as u32);

        for ((key, value), index) in source.into_iter().zip(0u32..) {
            let key = Rc::new(key);
            let index = Index::from(index);
            key_value.insert(key.clone(), value);
            index_key.insert(index, key.clone());
            key_index.insert(key, index);
        }

        IndexedMap {
            key_value,
            index_key,
            key_index,
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
        self.key_value
            .iter()
            .map(|(key, value)| (key.as_ref().clone(), value.clone()))
            .collect()
    }
}
