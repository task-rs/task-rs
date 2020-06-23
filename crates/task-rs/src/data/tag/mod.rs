pub mod data;
pub mod id;

pub use data::Data;
pub use id::Id;

pub(crate) mod entry;

use super::super::utils::IndexedMap;

pub type Map = IndexedMap<Id, Data>;
pub use super::super::utils::indexed_map::Index;
