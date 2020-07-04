use super::super::style;
use super::{TaskStatusFilter, TaskStatusFilterMessage, ThemeSwitcher, ThemeSwitcherMessage};
use iced::*;

pub struct Header<'a, Theme> {
    pub task_status_filter: TaskStatusFilter<'a, Theme>,
    pub theme_switcher: ThemeSwitcher<'a, Theme>,
}

impl<'a, Theme> Header<'a, Theme>
where
    Theme: style::Theme + Copy,
{
    pub fn view(self) -> Element<'a, Message> {
        Row::new()
            .push(Text::new("TaskRs").size(40))
            .push(
                self.task_status_filter
                    .view()
                    .map(Message::SetTaskStatusFilter),
            )
            .push(self.theme_switcher.view().map(Message::SetDarkMode))
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTaskStatusFilter(TaskStatusFilterMessage),
    SetDarkMode(ThemeSwitcherMessage),
}
