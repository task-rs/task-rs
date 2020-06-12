use super::{controller::update, view::view, Message, Model};
use iced::{executor, Application, Command, Element};

impl Application for Model {
    type Message = Message;
    type Flags = Model;
    type Executor = executor::Default;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (flags, Command::none())
    }

    fn title(&self) -> String {
        self.title.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update(self, message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        view(self)
    }
}
