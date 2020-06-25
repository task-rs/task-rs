use super::BidirectionalMap;
use core::borrow::Borrow;
use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Overwritten<Left, Right> {
    Both(Rc<Left>, Rc<Right>),
    LeftOnly(Rc<Left>),
    RightOnly(Rc<Right>),
    Neither,
}

impl<Left, Right> BidirectionalMap<Left, Right>
where
    Left: Ord,
    Right: Ord,
{
    pub fn inverse(self) -> BidirectionalMap<Right, Left> {
        BidirectionalMap {
            lr: self.rl,
            rl: self.lr,
        }
    }

    pub fn left_to_right_map(&self) -> &BTreeMap<Rc<Left>, Rc<Right>> {
        &self.lr
    }

    pub fn right_to_left_map(&self) -> &BTreeMap<Rc<Right>, Rc<Left>> {
        &self.rl
    }

    fn prv_remove(
        &mut self,
        left: impl Borrow<Left>,
        right: impl Borrow<Right>,
    ) -> Overwritten<Left, Right> {
        match (
            self.lr.remove(left.borrow()),
            self.rl.remove(right.borrow()),
        ) {
            (Some(removed_right), Some(removed_left)) => {
                self.lr.remove(&removed_left);
                self.rl.remove(&removed_right);
                Overwritten::Both(removed_left, removed_right)
            }
            (Some(removed_right), None) => {
                self.rl.remove(&removed_right);
                Overwritten::RightOnly(removed_right)
            }
            (None, Some(removed_left)) => {
                self.lr.remove(&removed_left);
                Overwritten::LeftOnly(removed_left)
            }
            (None, None) => Overwritten::Neither,
        }
    }

    fn prv_insert_unchecked(&mut self, left: Rc<Left>, right: Rc<Right>) {
        self.lr.insert(left.clone(), right.clone());
        self.rl.insert(right, left);
    }

    pub fn insert(&mut self, left: Rc<Left>, right: Rc<Right>) -> Overwritten<Left, Right> {
        let result = self.prv_remove(left.clone(), right.clone());
        self.prv_insert_unchecked(left, right);
        result
    }

    pub fn remove_by_left(&mut self, left: impl Borrow<Left>) -> Option<Rc<Right>> {
        if let Some(removed_right) = self.lr.remove(left.borrow()) {
            self.rl.remove(&removed_right);
            Some(removed_right)
        } else {
            None
        }
    }

    pub fn remove_by_right(&mut self, right: impl Borrow<Right>) -> Option<Rc<Left>> {
        if let Some(removed_left) = self.rl.remove(right.borrow()) {
            self.lr.remove(&removed_left);
            Some(removed_left)
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.lr.clear();
        self.rl.clear();
    }
}
