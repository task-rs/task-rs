pub mod state;

pub use state::State;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInterface {
    pub state: State,
}
