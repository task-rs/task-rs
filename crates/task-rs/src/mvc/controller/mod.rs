use super::{
    model::{view::tasks::FilterMethod, Theme},
    Message, Model,
};
use iced::Command;

pub fn new(model: Model) -> (Model, Command<Message>) {
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    match message {
        Message::MultipleActions(x) => {
            for x in x {
                update(model, x);
            }
        }
        Message::Warn(x) => println!("WARNING: {}", x),
        Message::SetTaskStatusFilter(x) => model.ui_state.details.task_status_filter = x,
        Message::SetDarkMode(x) => {
            model.ui_state.theme = if x { Theme::Dark } else { Theme::Light }
        }
        Message::SetTaskFilterMethod(x) => model.ui_state.view.tasks.filter_method = x,
        Message::FilterTasksBySingleTag(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::SingleTag;
            model.ui_state.view.tasks.single_tag = x;
        }
        Message::AddTagToMultipleTags(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            model.ui_state.view.tasks.multiple_tags.insert(x);
        }
        Message::RemoveTagFromMultipleTags(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            model.ui_state.view.tasks.multiple_tags.remove(&x);
        }
    }

    Command::none()
}
