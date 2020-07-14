use super::super::super::{data::Status, sizes::main::SUB_TASK_INDENT};
use super::{Message, TaskItem};
use iced::*;
use pipe_trait::*;

impl TaskItem {
    pub fn view(self) -> Element<'static, Message> {
        let indent_size = self
            .address
            .as_ref()
            .len()
            .pipe(|x| x as u16 * SUB_TASK_INDENT)
            .pipe(Length::Units);

        let indentation = Text::new("")
            .width(indent_size)
            .height(Length::Units(SUB_TASK_INDENT));

        let checkbox = Checkbox::new(
            match self.status {
                Status::Active => false,
                Status::Completed => true,
            },
            self.summary.clone(),
            move |is_checked| {
                Message::SetStatus(
                    self.address.as_ref().clone(),
                    if is_checked {
                        Status::Completed
                    } else {
                        Status::Active
                    },
                )
            },
        );

        Row::new().push(indentation).push(checkbox).into()
    }
}
