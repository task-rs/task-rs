use super::super::components::controls;
use super::{
    model::{view::tasks::FilterMethod, Theme},
    Message, Model,
};
use iced::*;
use pipe_trait::*;
use std::collections::{BTreeMap, BTreeSet};

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
        Message::MultipleActions(messages) => {
            for message in messages {
                update(model, message);
            }
        }
        Message::Warn(warning) => println!("WARNING: {}", warning),
        Message::SetTaskStatusFilter(task_status_filter) => {
            model.ui_state.details.task_status_filter = task_status_filter
        }
        Message::SetDarkMode(dark_mode) => {
            model.ui_state.theme = if dark_mode { Theme::Dark } else { Theme::Light }
        }
        Message::SetTaskFilterMethod(filter_method) => {
            model.ui_state.view.tasks.filter_method = filter_method;
            if filter_method == FilterMethod::All {
                model.ui_state.view.tasks.single_tag = None;
            }
            if model.ui_state.view.tasks.single_tag.is_none()
                && filter_method != FilterMethod::MultipleTags
            {
                model.ui_state.view.tasks.filter_method = FilterMethod::All;
            }
        }
        Message::FilterTasksBySingleTag(tag_index) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::SingleTag;
            model.ui_state.view.tasks.single_tag = lookup_tag_id!(tag_index).cloned();
        }
        Message::AddTagToMultipleTags(tag_index) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(tag_index) {
                model.ui_state.view.tasks.multiple_tags.insert(id.clone());
            }
        }
        Message::RemoveTagFromMultipleTags(tag_index) => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(tag_index) {
                model.ui_state.view.tasks.multiple_tags.remove(id);
            }
        }
        Message::CheckAllOfMultipleTags => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            model
                .ui_state
                .view
                .tasks
                .multiple_tags
                .extend(model.data.tags.iter().map(|(id, _)| id.clone()))
        }
        Message::UncheckAllOfMultipleTags => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            model.ui_state.view.tasks.multiple_tags = BTreeSet::new();
        }
        Message::InvertAllOfMultipleTags => {
            model.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            for (id, _) in model.data.tags.iter() {
                let multiple_tags = &mut model.ui_state.view.tasks.multiple_tags;
                if multiple_tags.contains(id) {
                    multiple_tags.remove(id);
                } else {
                    multiple_tags.insert(id.clone());
                }
            }
        }
    }

    Command::none()
}

fn init_update(model: &mut Model) {
    model.controls.tag_list = model
        .data
        .tags
        .iter_index()
        .map(|(index, _)| (index, button::State::default()))
        .collect::<BTreeMap<_, _>>()
        .pipe(controls::TagList);

    model.controls.task_list = model
        .data
        .tasks
        .iter()
        .enumerate()
        .map(|(index, task)| controls::TaskItem {
            task_address: vec![index],
            task_status: task.status,
            task_summary: task.summary.clone(),
        })
        .collect::<Vec<_>>()
        .pipe(|tasks| controls::TaskList { tasks });
}
