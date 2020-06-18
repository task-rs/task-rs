use iced::button::State;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub all: State,
    pub active: State,
    pub completed: State,
}
