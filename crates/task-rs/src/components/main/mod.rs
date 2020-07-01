pub mod message;
pub mod model;

pub use message::Message;
pub use model::Model;

use super::super::style::Theme;
use super::{
    task_status_filter, Header, Sidebar, TaskListMessage, TaskStatusFilter, ThemeSwitcher,
};
use iced::*;
use model::view::tasks::FilterMethod;
use pipe_trait::*;

pub use Model as Main;

impl Main {
    pub fn view(&mut self) -> Element<'_, Message> {
        let theme = &self.ui_state.theme;
        let task_status_filter = self.ui_state.details.task_status_filter;

        Column::new()
            .push(Header {
                task_status_filter: TaskStatusFilter {
                    controls: &mut self.controls.task_status_filter,
                    actual_value: task_status_filter,
                    get_message: Message::SetTaskStatusFilter,
                    theme,
                },
                theme_switcher: ThemeSwitcher {
                    dark_mode: self.ui_state.theme == model::Theme::Dark,
                    get_message: Message::SetDarkMode,
                    controls: &mut self.controls.theme_switcher,
                    theme,
                },
            })
            .push(
                Row::new()
                    .push(Sidebar {
                        tags: &self.data.tags,
                        task_view: &self.ui_state.view.tasks,
                        set_task_filter_method_to_all: Message::SetTaskFilterMethod(
                            FilterMethod::All,
                        ),
                        set_task_filter_method_to_single_tag: Message::SetTaskFilterMethod(
                            FilterMethod::SingleTag,
                        ),
                        set_task_filter_method_to_multiple_tags: Message::SetTaskFilterMethod(
                            FilterMethod::MultipleTags,
                        ),
                        tag_filter_method_controls: &mut self.controls.tag_filter_method,
                        single_tag: if let Some(id) = &self.ui_state.view.tasks.single_tag {
                            self.data.tags.get_index_by_key(id)
                        } else {
                            None
                        },
                        filter_tasks_by_single_tag: Message::FilterTasksBySingleTag,
                        add_tag_to_multiple_tags: Message::AddTagToMultipleTags,
                        remove_tag_from_multiple_tags: Message::RemoveTagFromMultipleTags,
                        check_all_of_multiple_tags: Message::CheckAllOfMultipleTags,
                        uncheck_all_of_multiple_tags: Message::UncheckAllOfMultipleTags,
                        invert_all_of_multiple_tags: Message::InvertAllOfMultipleTags,
                        tag_list_controls: &mut self.controls.tag_list,
                        theme,
                    })
                    .push(
                        self.controls
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

mod stylesheets;
