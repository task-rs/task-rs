use super::components::{Main, MainMessage};
use iced::{executor, Application, Command, Element};

impl Application for Main {
    type Message = MainMessage;
    type Flags = Main;
    type Executor = executor::Default;

    fn new(mut flags: Self::Flags) -> (Self, Command<Self::Message>) {
        flags.refresh();
        (flags, Command::none())
    }

    fn title(&self) -> String {
        self.ui_state.title.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.update(message)
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        self.view()
    }
}
