use super::{
    model::{view::tasks::FilterMethod, Theme},
    Main, Message,
};
use iced::*;
use std::collections::BTreeSet;

pub fn update(main: &mut Main, message: Message) -> Command<Message> {
    main.refresh();

    macro_rules! lookup_tag_id {
        ($index:ident) => {
            main.data.tags.get_key_by_index($index).map(AsRef::as_ref)
        };
    }

    match message {
        Message::MultipleActions(messages) => {
            for message in messages {
                update(main, message);
            }
        }
        Message::Warn(warning) => println!("WARNING: {}", warning),
        Message::SetTaskStatusFilter(task_status_filter) => {
            main.ui_state.details.task_status_filter = task_status_filter
        }
        Message::SetDarkMode(dark_mode) => {
            main.ui_state.theme = if dark_mode { Theme::Dark } else { Theme::Light }
        }
        Message::SetTaskFilterMethod(filter_method) => {
            main.ui_state.view.tasks.filter_method = filter_method;
            if filter_method == FilterMethod::All {
                main.ui_state.view.tasks.single_tag = None;
            }
            if main.ui_state.view.tasks.single_tag.is_none()
                && filter_method != FilterMethod::MultipleTags
            {
                main.ui_state.view.tasks.filter_method = FilterMethod::All;
            }
        }
        Message::FilterTasksBySingleTag(tag_index) => {
            main.ui_state.view.tasks.filter_method = FilterMethod::SingleTag;
            main.ui_state.view.tasks.single_tag = lookup_tag_id!(tag_index).cloned();
        }
        Message::AddTagToMultipleTags(tag_index) => {
            main.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(tag_index) {
                main.ui_state.view.tasks.multiple_tags.insert(id.clone());
            }
        }
        Message::RemoveTagFromMultipleTags(tag_index) => {
            main.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            if let Some(id) = lookup_tag_id!(tag_index) {
                main.ui_state.view.tasks.multiple_tags.remove(id);
            }
        }
        Message::CheckAllOfMultipleTags => {
            main.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            main.ui_state
                .view
                .tasks
                .multiple_tags
                .extend(main.data.tags.iter().map(|(id, _)| id.clone()))
        }
        Message::UncheckAllOfMultipleTags => {
            main.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            main.ui_state.view.tasks.multiple_tags = BTreeSet::new();
        }
        Message::InvertAllOfMultipleTags => {
            main.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
            for (id, _) in main.data.tags.iter() {
                let multiple_tags = &mut main.ui_state.view.tasks.multiple_tags;
                if multiple_tags.contains(id) {
                    multiple_tags.remove(id);
                } else {
                    multiple_tags.insert(id.clone());
                }
            }
        }
        Message::SetTaskStatus(address, status) => {
            println!("set task status {:?} {:?}", address, status);
        }
    }

    Command::none()
}
