use super::IndexedMap;
use serde::{de::DeserializeOwned, ser::Serialize};
use std::rc::Rc;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InsertResult<ReplacedValue> {
    Replaced(ReplacedValue),
    Added(u32),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RemoveResult<RemovedKey, RemovedValue> {
    Removed(RemovedKey, RemovedValue),
    Unchanged,
}

impl<Key, Value> IndexedMap<Key, Value>
where
    Key: Ord + Clone,
    Value: Clone,
{
    pub(crate) fn with(mut self, key: impl Into<Key>, value: Value) -> Self {
        self.insert_key(Rc::new(key.into()), value);
        self
    }

    pub(crate) fn from_yaml(yaml: &str) -> Self
    where
        Key: DeserializeOwned,
        Value: DeserializeOwned,
    {
        serde_yaml::from_str(yaml).expect("parse IndexedMap from yaml string")
    }

    pub(crate) fn to_yaml(&self) -> String
    where
        Key: Serialize,
        Value: Serialize,
    {
        serde_yaml::to_string(self).expect("dump IndexedMap as yaml string")
    }

    pub fn get_value_by_key(&self, key: &Rc<Key>) -> Option<&Value> {
        self.key_value.get(key)
    }

    pub fn get_value_by_index(&self, index: u32) -> Option<&Value> {
        let key = self.get_key_by_index(index)?;
        self.get_value_by_key(key)
    }

    pub fn get_key_by_index(&self, index: u32) -> Option<&Rc<Key>> {
        self.index_key.get(&index)
    }

    pub fn get_index_by_key(&self, key: &Rc<Key>) -> Option<u32> {
        self.key_index.get(key).cloned()
    }

    pub fn insert_key(&mut self, key: Rc<Key>, value: Value) -> InsertResult<Value> {
        if let Some(value) = self.key_value.insert(key.clone(), value) {
            InsertResult::Replaced(value)
        } else {
            let index = self.counter;
            self.counter += 1;
            self.key_index.insert(key.clone(), index);
            self.index_key.insert(index, key);
            InsertResult::Added(index)
        }
    }

    pub fn remove_key(&mut self, key: &Rc<Key>) -> RemoveResult<u32, Value> {
        if let Some(value) = self.key_value.remove(key) {
            let index = self
                .key_index
                .remove(key)
                .expect("remove (key, index) from key_index");
            self.index_key.remove(&index);
            RemoveResult::Removed(index, value)
        } else {
            RemoveResult::Unchanged
        }
    }

    pub fn replace_index(&mut self, index: u32, value: Value) -> Option<Value> {
        if let Some(key) = self.get_key_by_index(index).cloned() {
            if let InsertResult::Replaced(value) = self.insert_key(key, value) {
                Some(value)
            } else {
                panic!("invalid state")
            }
        } else {
            None
        }
    }

    pub fn remove_index(&mut self, index: u32) -> RemoveResult<Rc<Key>, Value> {
        if let Some(key) = self.get_key_by_index(index).cloned() {
            if let RemoveResult::Removed(_, value) = self.remove_key(&key) {
                return RemoveResult::Removed(key, value);
            }
        }

        RemoveResult::Unchanged
    }
}
