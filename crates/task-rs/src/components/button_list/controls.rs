use iced::*;
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Controls<Key>(pub BTreeMap<Key, button::State>)
where
    Key: Ord;
