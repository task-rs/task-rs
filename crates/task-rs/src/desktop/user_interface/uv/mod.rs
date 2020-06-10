use super::{Message, UserInterface};
use iced::{Element, Sandbox};

impl Sandbox for UserInterface {
    type Message = Message;

    fn new() -> Self {
        UserInterface::default()
    }

    fn title(&self) -> String {
        "TaskRs".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        update::update(self, message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        view::view(self)
    }
}

mod update;
mod view;
