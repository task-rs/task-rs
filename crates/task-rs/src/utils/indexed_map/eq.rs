use super::IndexedMap;

impl<Key, Value> PartialEq for IndexedMap<Key, Value>
where
    Key: Ord + Clone + Eq,
    Value: Clone + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

impl<Key, Value> Eq for IndexedMap<Key, Value>
where
    Key: Ord + Clone + Eq,
    Value: Clone + Eq,
{
}
