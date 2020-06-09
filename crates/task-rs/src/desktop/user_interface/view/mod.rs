pub mod tasks;

pub use tasks::Tasks;

use super::super::super::default_enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum View {
    Tasks(Tasks),
    Config,
    About,
}

default_enum!(View::Config);
