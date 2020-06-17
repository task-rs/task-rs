use super::{task_status_filter::TaskStatusFilter, theme};
use iced::*;

pub struct Header<'ctrl, Theme, Message>
where
    Theme: theme::Theme + Copy,
{
    pub task_status_filter: TaskStatusFilter<'ctrl, Theme, Message>,
}

impl<'ctrl, Theme, Message> Into<Element<'ctrl, Message>> for Header<'ctrl, Theme, Message>
where
    Theme: theme::Theme + Copy,
    Message: Clone + 'ctrl,
{
    fn into(self) -> Element<'ctrl, Message> {
        Row::new()
            .push(Text::new("TaskRs").size(40))
            .push(self.task_status_filter)
            .into()
    }
}
