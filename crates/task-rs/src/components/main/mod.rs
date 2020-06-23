use super::super::{
    data::TagMapIndex,
    mvc::{model, Model},
    style::Theme,
};
use super::{task_status_filter, Header, Sidebar, TaskStatusFilter, ThemeSwitcher};
use iced::*;
use pipe_trait::*;

pub struct Main<'a, Message> {
    pub model: &'a mut Model,
    pub set_task_status_filter: fn(task_status_filter::Value) -> Message,
    pub set_dark_mode: fn(bool) -> Message,
    pub set_tag_filter_method_to_all: Message,
    pub set_task_filter_method_to_single_tag: Message,
    pub set_task_filter_method_to_multiple_tags: Message,
    pub filter_tasks_by_single_tag: fn(TagMapIndex) -> Message,
    pub add_tag_to_multiple_tags: fn(TagMapIndex) -> Message,
    pub remove_tag_from_multiple_tags: fn(TagMapIndex) -> Message,
}

impl<'a, Message> Into<Element<'a, Message>> for Main<'a, Message>
where
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let theme = &self.model.ui_state.theme;
        let task_status_filter = self.model.ui_state.details.task_status_filter;

        Column::new()
            .push(Header {
                task_status_filter: TaskStatusFilter {
                    controls: &mut self.model.controls.task_status_filter,
                    actual_value: task_status_filter,
                    get_message: self.set_task_status_filter,
                    theme,
                },
                theme_switcher: ThemeSwitcher {
                    dark_mode: self.model.ui_state.theme == model::Theme::Dark,
                    get_message: self.set_dark_mode,
                    controls: &mut self.model.controls.theme_switcher,
                    theme,
                },
            })
            .push(Row::new().push(Sidebar {
                tags: &self.model.data.tags,
                task_view: &self.model.ui_state.view.tasks,
                set_task_filter_method_to_all: self.set_tag_filter_method_to_all,
                set_task_filter_method_to_single_tag: self.set_task_filter_method_to_single_tag,
                set_task_filter_method_to_multiple_tags:
                    self.set_task_filter_method_to_multiple_tags,
                tag_filter_method_controls: &mut self.model.controls.tag_filter_method,
                single_tag: self.model.ui_state.view.tasks.single_tag.clone(), // TODO: optimize
                filter_tasks_by_single_tag: self.filter_tasks_by_single_tag,
                add_tag_to_multiple_tags: self.add_tag_to_multiple_tags,
                remove_tag_from_multiple_tags: self.remove_tag_from_multiple_tags,
                tag_list_controls: &mut self.model.controls.tag_list,
                theme,
            }))
            .pipe(Container::new)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(stylesheets::Container(theme.style()))
            .into()
    }
}

mod stylesheets;
