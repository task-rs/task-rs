use super::{task_status_filter::TaskStatusFilter, theme};
use iced::*;

pub struct Header<'a, Theme, Message>
where
    Theme: theme::Theme + Copy,
{
    pub task_status_filter: TaskStatusFilter<'a, Theme, Message>,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for Header<'a, Theme, Message>
where
    Theme: theme::Theme + Copy,
    Message: Clone + 'a,
{
    fn into(self) -> Element<'a, Message> {
        Row::new()
            .push(Text::new("TaskRs").size(40))
            .push(self.task_status_filter)
            .into()
    }
}
