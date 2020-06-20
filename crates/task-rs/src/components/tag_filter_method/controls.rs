use iced::*;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub filter_method_single_tag: button::State,
    pub filter_method_multiple_tags: button::State,
    pub all_button: button::State,
}
