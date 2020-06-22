use super::super::style;
use super::{task_status_filter::TaskStatusFilter, ThemeSwitcher};
use iced::*;

pub struct Header<'a, Theme, Message> {
    pub task_status_filter: TaskStatusFilter<'a, Theme, Message>,
    pub theme_switcher: ThemeSwitcher<'a, Theme, Message>,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for Header<'a, Theme, Message>
where
    Theme: style::Theme + Copy,
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        Row::new()
            .push(Text::new("TaskRs").size(40))
            .push(self.task_status_filter)
            .push(self.theme_switcher)
            .into()
    }
}
