pub mod tasks;

pub use tasks::Tasks;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize)]
pub enum View {
    Tasks(Tasks),
    #[default]
    Config,
    About,
}
