use super::BidirectionalMap;
use core::iter::FromIterator;
use pipe_trait::*;
use std::rc::Rc;

impl<Left, Right> FromIterator<(Rc<Left>, Rc<Right>)> for BidirectionalMap<Left, Right>
where
    Left: Ord,
    Right: Ord,
{
    fn from_iter<Iter: IntoIterator<Item = (Rc<Left>, Rc<Right>)>>(iter: Iter) -> Self {
        let mut result = BidirectionalMap::default();

        for (left, right) in iter {
            result.insert(left, right);
        }

        result
    }
}

impl<Left, Right> FromIterator<(Left, Right)> for BidirectionalMap<Left, Right>
where
    Left: Ord,
    Right: Ord,
{
    fn from_iter<Iter: IntoIterator<Item = (Left, Right)>>(iter: Iter) -> Self {
        iter.into_iter()
            .map(|(left, right)| (Rc::new(left), Rc::new(right)))
            .pipe(FromIterator::from_iter)
    }
}
