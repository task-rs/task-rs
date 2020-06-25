pub mod methods;

pub use methods::Overwritten;

use std::{collections::BTreeMap, rc::Rc};

#[derive(Debug, Clone)]
pub struct BidirectionalMap<Left, Right>
where
    Left: Ord,
    Right: Ord,
{
    lr: BTreeMap<Rc<Left>, Rc<Right>>,
    rl: BTreeMap<Rc<Right>, Rc<Left>>,
}

mod default;
mod eq;
mod from_iter;

#[cfg(test)]
mod tests;
