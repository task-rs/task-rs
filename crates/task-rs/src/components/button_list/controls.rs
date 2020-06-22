use iced::*;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Controls<Key>(pub BTreeMap<Key, button::State>)
where
    Key: Ord;

impl<Key> Controls<Key>
where
    Key: Ord,
{
    pub(crate) fn new(map: BTreeMap<Key, button::State>) -> Self {
        Controls(map)
    }
}
