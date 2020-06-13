use super::super::data::Data;
use super::{controller::update, view::view, Message, Model, UiState};
use iced::{executor, Application, Command, Element};

impl Application for Model {
    type Message = Message;
    type Flags = UiState;
    type Executor = executor::Default;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let data = Data::default();
        let model = Model {
            data,
            ui_state: flags,
        };
        (model, Command::none())
    }

    fn title(&self) -> String {
        self.ui_state.title.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        update(self, message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        view(self)
    }
}
