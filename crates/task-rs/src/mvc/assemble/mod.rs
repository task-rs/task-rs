use super::{controller::update, view::view, Message, Model};
use iced::{Element, Sandbox};

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Model::default()
    }

    fn title(&self) -> String {
        "TaskRs".to_owned()
    }

    fn update(&mut self, message: Self::Message) {
        update(self, message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        view(self)
    }
}
