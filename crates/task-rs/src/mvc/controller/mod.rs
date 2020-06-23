use super::super::components::controls;
use super::{
    model::{view::tasks::FilterMethod, Theme},
    Message, Model,
};
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

pub fn new(mut model: Model) -> (Model, Command<Message>) {
    init_update(&mut model);
    (model, Command::none())
}

pub fn update(model: &mut Model, message: Message) -> Command<Message> {
    init_update(model);

    macro_rules! lookup_tag_id {
        ($index:ident) => {
            model.data.tags.get_key_by_index($index).map(AsRef::as_ref)
        };
    }

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
            model.ui_state.view.tasks.single_tag = lookup_tag_id!(x).cloned();
        }
        Message::AddTagToMultipleTags(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(x) {
                model.ui_state.view.tasks.multiple_tags.insert(id.clone());
            }
        }
        Message::RemoveTagFromMultipleTags(x) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(x) {
                model.ui_state.view.tasks.multiple_tags.remove(id);
            }
        }
    }

    Command::none()
}

fn init_update(model: &mut Model) {
    model.controls.tag_list = model
        .data
        .tags
        .iter()
        .map(|(id, _)| (id.clone(), button::State::default()))
        .collect::<BTreeMap<_, _>>()
        .pipe(controls::TagList::new);
}
