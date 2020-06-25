use super::BidirectionalMap;

impl<Left, Right> PartialEq for BidirectionalMap<Left, Right>
where
    Left: Ord + PartialEq,
    Right: Ord + PartialEq,
{
    fn eq(&self, other: &BidirectionalMap<Left, Right>) -> bool {
        self.lr == other.lr
    }
}

impl<Left, Right> Eq for BidirectionalMap<Left, Right>
where
    Left: Ord + Eq,
    Right: Ord + Eq,
{
}
