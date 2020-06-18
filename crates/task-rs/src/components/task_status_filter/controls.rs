use iced::button::State;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub(crate) all: State,
    pub(crate) active: State,
    pub(crate) completed: State,
}
