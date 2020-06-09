pub mod tasks;

pub use tasks::Tasks;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum View {
    Tasks(Tasks),
    Config,
    About,
}

impl Default for View {
    fn default() -> Self {
        View::Config
    }
}
