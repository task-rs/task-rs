use super::{
    model::{view::tasks::FilterMethod, Theme},
    Main, Message,
};
use iced::*;
use std::collections::BTreeSet;

impl Main {
    pub fn update(&mut self, message: Message) -> Command<Message> {
        macro_rules! lookup_tag_id {
            ($index:ident) => {
                self.data.tags.get_key_by_index($index).map(AsRef::as_ref)
            };
        }

        match message {
            Message::MultipleActions(messages) => {
                for message in messages {
                    self.update(message);
                }
            }
            Message::Warn(warning) => println!("WARNING: {}", warning),
            Message::SetTaskStatusFilter(task_status_filter) => {
                self.ui_state.details.task_status_filter = task_status_filter
            }
            Message::SetDarkMode(dark_mode) => {
                self.ui_state.theme = if dark_mode { Theme::Dark } else { Theme::Light }
            }
            Message::SetTaskFilterMethod(filter_method) => {
                self.ui_state.view.tasks.filter_method = filter_method;
                if filter_method == FilterMethod::All {
                    self.ui_state.view.tasks.single_tag = None;
                }
                if self.ui_state.view.tasks.single_tag.is_none()
                    && filter_method != FilterMethod::MultipleTags
                {
                    self.ui_state.view.tasks.filter_method = FilterMethod::All;
                }
            }
            Message::FilterTasksBySingleTag(tag_index) => {
                self.ui_state.view.tasks.filter_method = FilterMethod::SingleTag;
                self.ui_state.view.tasks.single_tag = lookup_tag_id!(tag_index).cloned();
            }
            Message::AddTagToMultipleTags(tag_index) => {
                self.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
                if let Some(id) = lookup_tag_id!(tag_index) {
                    self.ui_state.view.tasks.multiple_tags.insert(id.clone());
                }
            }
            Message::RemoveTagFromMultipleTags(tag_index) => {
                self.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
                if let Some(id) = lookup_tag_id!(tag_index) {
                    self.ui_state.view.tasks.multiple_tags.remove(id);
                }
            }
            Message::CheckAllOfMultipleTags => {
                self.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
                self.ui_state
                    .view
                    .tasks
                    .multiple_tags
                    .extend(self.data.tags.iter().map(|(id, _)| id.clone()))
            }
            Message::UncheckAllOfMultipleTags => {
                self.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
                self.ui_state.view.tasks.multiple_tags = BTreeSet::new();
            }
            Message::InvertAllOfMultipleTags => {
                self.ui_state.view.tasks.filter_method = FilterMethod::MultipleTags;
                for (id, _) in self.data.tags.iter() {
                    let multiple_tags = &mut self.ui_state.view.tasks.multiple_tags;
                    if multiple_tags.contains(id) {
                        multiple_tags.remove(id);
                    } else {
                        multiple_tags.insert(id.clone());
                    }
                }
            }
            Message::SetTaskStatus(address, status) => {
                set_task_status::set_task_status(&mut self.data.tasks, &address, status);
            }
        }

        self.refresh();

        Command::none()
    }
}

mod find_task;
mod set_task_status;
