use super::IndexedMap;

impl<Key, Value> PartialEq for IndexedMap<Key, Value>
where
    Key: Ord + Clone + Eq,
    Value: Clone + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.key_value == other.key_value
    }
}

impl<Key, Value> Eq for IndexedMap<Key, Value>
where
    Key: Ord + Clone + Eq,
    Value: Clone + Eq,
{
}
