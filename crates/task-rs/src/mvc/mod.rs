pub mod model;
pub mod view;

pub use message::Message;
pub use model::{Model, UiState};

mod assemble;
mod controller;
mod message;
