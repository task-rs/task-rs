use super::BidirectionalMap;

impl<Key, Value> Default for BidirectionalMap<Key, Value>
where
    Key: Ord,
    Value: Ord,
{
    fn default() -> Self {
        BidirectionalMap {
            lr: Default::default(),
            rl: Default::default(),
        }
    }
}
