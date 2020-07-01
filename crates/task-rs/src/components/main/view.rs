use super::super::super::style::Theme;
use super::super::{Header, Sidebar, TaskListMessage, TaskStatusFilter, ThemeSwitcher};
use super::{
    model::{self, view::tasks::FilterMethod},
    stylesheets, Main, Message,
};
use iced::*;
use pipe_trait::*;

pub fn view(main: &mut Main) -> Element<'_, Message> {
    let theme = &main.ui_state.theme;
    let task_status_filter = main.ui_state.details.task_status_filter;

    Column::new()
        .push(Header {
            task_status_filter: TaskStatusFilter {
                controls: &mut main.controls.task_status_filter,
                actual_value: task_status_filter,
                get_message: Message::SetTaskStatusFilter,
                theme,
            },
            theme_switcher: ThemeSwitcher {
                dark_mode: main.ui_state.theme == model::Theme::Dark,
                get_message: Message::SetDarkMode,
                controls: &mut main.controls.theme_switcher,
                theme,
            },
        })
        .push(
            Row::new()
                .push(Sidebar {
                    tags: &main.data.tags,
                    task_view: &main.ui_state.view.tasks,
                    set_task_filter_method_to_all: Message::SetTaskFilterMethod(FilterMethod::All),
                    set_task_filter_method_to_single_tag: Message::SetTaskFilterMethod(
                        FilterMethod::SingleTag,
                    ),
                    set_task_filter_method_to_multiple_tags: Message::SetTaskFilterMethod(
                        FilterMethod::MultipleTags,
                    ),
                    tag_filter_method_controls: &mut main.controls.tag_filter_method,
                    single_tag: if let Some(id) = &main.ui_state.view.tasks.single_tag {
                        main.data.tags.get_index_by_key(id)
                    } else {
                        None
                    },
                    filter_tasks_by_single_tag: Message::FilterTasksBySingleTag,
                    add_tag_to_multiple_tags: Message::AddTagToMultipleTags,
                    remove_tag_from_multiple_tags: Message::RemoveTagFromMultipleTags,
                    check_all_of_multiple_tags: Message::CheckAllOfMultipleTags,
                    uncheck_all_of_multiple_tags: Message::UncheckAllOfMultipleTags,
                    invert_all_of_multiple_tags: Message::InvertAllOfMultipleTags,
                    tag_list_controls: &mut main.controls.tag_list,
                    theme,
                })
                .push(
                    main.controls
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
