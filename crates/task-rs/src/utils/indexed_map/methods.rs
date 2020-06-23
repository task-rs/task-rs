use super::{Index, IndexedMap};
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InsertResult<ReplacedValue> {
    Replaced(ReplacedValue),
    Added(Index),
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
    pub fn into_btreemap(self) -> BTreeMap<Key, Value> {
        self.into()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Key, &Value)> {
        self.entries
            .iter()
            .map(|(key, value)| (key.as_ref(), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Key, &mut Value)> {
        self.entries
            .iter_mut()
            .map(|(key, value)| (key.as_ref(), value))
    }

    pub fn iter_index(&mut self) -> impl Iterator<Item = (Index, &Key)> {
        self.indices
            .iter()
            .map(|(key, index)| (*index, key.as_ref()))
    }

    pub fn get_value_by_key(&self, key: &Rc<Key>) -> Option<&Value> {
        self.entries.get(key)
    }

    pub fn get_value_by_index(&self, index: Index) -> Option<&Value> {
        let key = self.get_key_by_index(index)?;
        self.get_value_by_key(key)
    }

    pub fn get_key_by_index(&self, index: Index) -> Option<&Rc<Key>> {
        self.indices.get_by_right(&index)
    }

    pub fn get_index_by_key(&self, key: &Rc<Key>) -> Option<Index> {
        self.indices.get_by_left(key).cloned()
    }

    pub fn insert_key(&mut self, key: Rc<Key>, value: Value) -> InsertResult<Value> {
        if let Some(value) = self.entries.insert(key.clone(), value) {
            InsertResult::Replaced(value)
        } else {
            let index = self.counter;
            *self.counter.as_mut() += 1;
            self.indices.insert(key, index);
            InsertResult::Added(index)
        }
    }

    pub fn remove_key(&mut self, key: &Rc<Key>) -> RemoveResult<Index, Value> {
        if let Some(value) = self.entries.remove(key) {
            let (_, index) = self
                .indices
                .remove_by_left(key)
                .expect("remove (key, index) from indices");
            RemoveResult::Removed(index, value)
        } else {
            RemoveResult::Unchanged
        }
    }

    pub fn replace_index(&mut self, index: Index, value: Value) -> Option<Value> {
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

    pub fn remove_index(&mut self, index: Index) -> RemoveResult<Rc<Key>, Value> {
        if let Some(key) = self.get_key_by_index(index).cloned() {
            if let RemoveResult::Removed(_, value) = self.remove_key(&key) {
                return RemoveResult::Removed(key, value);
            }
        }

        RemoveResult::Unchanged
    }
}
