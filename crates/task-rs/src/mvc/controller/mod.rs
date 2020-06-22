use super::super::components::controls;
use super::{
    model::{view::tasks::FilterMethod, Theme},
    Message, Model,
};
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

pub fn new(model: Model) -> (Model, Command<Message>) {
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    model.controls.tag_list = model
        .data
        .tags
        .iter()
        .map(|(id, _)| (id.clone(), button::State::default()))
        .collect::<BTreeMap<_, _>>()
        .pipe(controls::TagList::new);

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
        Message::SetTaskFilterMethod(x) => {
            model.ui_state.view.tasks.filter_method = x;
            if x == FilterMethod::All {
                model.ui_state.view.tasks.single_tag = None;
            }
            if model.ui_state.view.tasks.single_tag.is_none() && x != FilterMethod::MultipleTags {
                model.ui_state.view.tasks.filter_method = FilterMethod::All;
            }
        }
        Message::FilterTasksBySingleTag(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::SingleTag;
            model.ui_state.view.tasks.single_tag = Some(x);
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
