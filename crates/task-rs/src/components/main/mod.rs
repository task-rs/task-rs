pub mod message;
pub mod model;

pub use message::Message;
pub use model::Model;

pub type Main = Model;

mod refresh;
mod stylesheets;
mod update;
mod view;
