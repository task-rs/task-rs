pub mod message;
pub mod model;
pub mod view;

pub use message::Message;
pub use model::Model;

use iced::*;

pub use Model as Main;

impl Main {
    pub fn view(&mut self) -> Element<'_, Message> {
        view::view(self)
    }
}

mod stylesheets;
