pub mod model;

pub use model::Model;

use super::super::{
    data::{Status, TagMapIndex},
    style::Theme,
};
use super::{
    task_status_filter, Header, Sidebar, TaskListMessage, TaskStatusFilter, ThemeSwitcher,
};
use iced::*;
use pipe_trait::*;

pub struct Main<'a> {
    pub model: &'a mut Model,
}

impl<'a> Main<'a> {
    pub fn view(self) -> Element<'a, Message> {
        let theme = &self.model.ui_state.theme;
        let task_status_filter = self.model.ui_state.details.task_status_filter;

        Column::new()
            .push(Header {
                task_status_filter: TaskStatusFilter {
                    controls: &mut self.model.controls.task_status_filter,
                    actual_value: task_status_filter,
                    get_message: Message::SetTaskStatusFilter,
                    theme,
                },
                theme_switcher: ThemeSwitcher {
                    dark_mode: self.model.ui_state.theme == model::Theme::Dark,
                    get_message: Message::SetDarkMode,
                    controls: &mut self.model.controls.theme_switcher,
                    theme,
                },
            })
            .push(
                Row::new()
                    .push(Sidebar {
                        tags: &self.model.data.tags,
                        task_view: &self.model.ui_state.view.tasks,
                        set_task_filter_method_to_all: Message::SetTaskFilterMethodToAll,
                        set_task_filter_method_to_single_tag:
                            Message::SetTaskFilterMethodToSingleTag,
                        set_task_filter_method_to_multiple_tags:
                            Message::SetTaskFilterMethodToMultipleTags,
                        tag_filter_method_controls: &mut self.model.controls.tag_filter_method,
                        single_tag: if let Some(id) = &self.model.ui_state.view.tasks.single_tag {
                            self.model.data.tags.get_index_by_key(id)
                        } else {
                            None
                        },
                        filter_tasks_by_single_tag: Message::FilterTasksBySingleTag,
                        add_tag_to_multiple_tags: Message::AddTagToMultipleTags,
                        remove_tag_from_multiple_tags: Message::RemoveTagFromMultipleTags,
                        check_all_of_multiple_tags: Message::CheckAllOfMultipleTags,
                        uncheck_all_of_multiple_tags: Message::UncheckAllOfMultipleTags,
                        invert_all_of_multiple_tags: Message::InvertAllOfMultipleTags,
                        tag_list_controls: &mut self.model.controls.tag_list,
                        theme,
                    })
                    .push(
                        self.model
                            .controls
                            .task_list
                            .view()
                            .map(move |message| match message {
                                TaskListMessage::SetStatus(address, status) => {
                                    Message::SetTaskStatus(address, status)
                                }
                            }),
                    ),
            )
            .pipe(Container::new)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(stylesheets::Container(theme.style()))
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTaskStatusFilter(task_status_filter::Value),
    SetDarkMode(bool),
    SetTaskFilterMethodToAll,
    SetTaskFilterMethodToSingleTag,
    SetTaskFilterMethodToMultipleTags,
    FilterTasksBySingleTag(TagMapIndex),
    AddTagToMultipleTags(TagMapIndex),
    RemoveTagFromMultipleTags(TagMapIndex),
    CheckAllOfMultipleTags,
    UncheckAllOfMultipleTags,
    InvertAllOfMultipleTags,
    SetTaskStatus(Vec<usize>, Status),
}

mod stylesheets;
