use iced::button::State;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub task_state_filter_all: State,
    pub task_state_filter_active: State,
    pub task_state_filter_completed: State,
}
