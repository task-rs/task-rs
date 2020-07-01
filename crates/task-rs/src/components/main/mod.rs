pub mod message;
pub mod model;
pub mod refresh;
pub mod update;
pub mod view;

pub use message::Message;
pub use model::Model;

use iced::*;

pub type Main = Model;

impl Main {
    pub fn refresh(&mut self) {
        refresh::refresh(self)
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        update::update(self, message)
    }

    pub fn view(&mut self) -> Element<'_, Message> {
        view::view(self)
    }
}

mod stylesheets;
