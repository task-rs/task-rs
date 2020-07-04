use super::super::super::style::Theme;
use super::super::{
    Header, HeaderMessage, Sidebar, SidebarMessage, TaskListMessage, TaskStatusFilter,
    ThemeSwitcher,
};
use super::{
    model::{self, view::tasks::FilterMethod},
    stylesheets, Main, Message,
};
use iced::*;
use pipe_trait::*;

impl Main {
    pub fn view(&mut self) -> Element<'_, Message> {
        let theme = &self.ui_state.theme;
        let task_status_filter = self.ui_state.details.task_status_filter;

        Column::new()
            .push(
                Header {
                    task_status_filter: TaskStatusFilter {
                        controls: &mut self.controls.task_status_filter,
                        actual_value: task_status_filter,
                        theme,
                    },
                    theme_switcher: ThemeSwitcher {
                        dark_mode: self.ui_state.theme == model::Theme::Dark,
                        controls: &mut self.controls.theme_switcher,
                        theme,
                    },
                }
                .view()
                .map(|message| match message {
                    HeaderMessage::SetTaskStatusFilter(message) => {
                        Message::SetTaskStatusFilter(message.0)
                    }
                    HeaderMessage::SetDarkMode(message) => Message::SetDarkMode(message.0),
                }),
            )
            .push(
                Row::new()
                    .push(
                        Sidebar {
                            tags: &self.data.tags,
                            task_view: &self.ui_state.view.tasks,
                            tag_filter_method_controls: &mut self.controls.tag_filter_method,
                            single_tag: if let Some(id) = &self.ui_state.view.tasks.single_tag {
                                self.data.tags.get_index_by_key(id)
                            } else {
                                None
                            },
                            tag_list_controls: &mut self.controls.tag_list,
                            theme,
                        }
                        .view()
                        .map(|message| match message {
                            SidebarMessage::SetTaskFilterMethodToAll => {
                                Message::SetTaskFilterMethod(FilterMethod::All)
                            }
                            SidebarMessage::SetTaskFilterMethodToSingleTag => {
                                Message::SetTaskFilterMethod(FilterMethod::SingleTag)
                            }
                            SidebarMessage::SetTaskFilterMethodToMultipleTags => {
                                Message::SetTaskFilterMethod(FilterMethod::MultipleTags)
                            }
                            SidebarMessage::FilterTaskBySingleTag(index) => {
                                Message::FilterTasksBySingleTag(index)
                            }
                            SidebarMessage::AddTagToMultipleTags(index) => {
                                Message::AddTagToMultipleTags(index)
                            }
                            SidebarMessage::RemoveTagFromMultipleTags(index) => {
                                Message::RemoveTagFromMultipleTags(index)
                            }
                            SidebarMessage::CheckAllOfMultipleTags => {
                                Message::CheckAllOfMultipleTags
                            }
                            SidebarMessage::UncheckAllOfMultipleTags => {
                                Message::UncheckAllOfMultipleTags
                            }
                            SidebarMessage::InvertAllOfMultipleTags => {
                                Message::InvertAllOfMultipleTags
                            }
                        }),
                    )
                    .push(self.controls.task_list.view().map(|message| match message {
                        TaskListMessage::SetStatus(address, status) => {
                            Message::SetTaskStatus(address, status)
                        }
                    })),
            )
            .pipe(Container::new)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(stylesheets::Container(theme.style()))
            .into()
    }
}
