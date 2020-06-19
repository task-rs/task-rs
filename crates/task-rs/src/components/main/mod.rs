use super::super::{
    data::Data,
    mvc::{model, UiState},
};
use super::{
    task_status_filter, Controls, Header, Sidebar, TaskStatusFilter, Theme, ThemeSwitcher,
};
use iced::*;
use pipe_trait::*;

pub struct Main<'a, Message> {
    pub data: &'a mut Data,
    pub ui_state: &'a mut UiState,
    pub controls: &'a mut Controls,
    pub set_task_status_filter: fn(task_status_filter::Value) -> Message,
    pub set_dark_mode: fn(bool) -> Message,
}

impl<'a, Message> Into<Element<'a, Message>> for Main<'a, Message>
where
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let theme = &self.ui_state.theme;
        let task_status_filter = self.ui_state.details.task_status_filter;

        Column::new()
            .push(Header {
                task_status_filter: TaskStatusFilter {
                    controls: &mut self.controls.task_status_filter,
                    actual_value: task_status_filter,
                    get_message: self.set_task_status_filter,
                    theme,
                },
                theme_switcher: ThemeSwitcher {
                    dark_mode: self.ui_state.theme == model::Theme::Dark,
                    get_message: self.set_dark_mode,
                    controls: &mut self.controls.theme_switcher,
                    theme,
                },
            })
            .push(Row::new().push(Sidebar(&self.data.tags)))
            .pipe(Container::new)
            .style(stylesheets::Container(theme.style()))
            .into()
    }
}

mod stylesheets;
