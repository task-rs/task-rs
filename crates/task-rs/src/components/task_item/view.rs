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

        let summary = Text::new(self.summary.clone());

        let tags = {
            let mut tags = Row::new();

            for tag in &self.tags {
                let label = format!("#{}", &tag.0);
                let label = Text::new(label);
                tags = tags.push(label);
            }

            tags
        };

        let checkbox = Checkbox::new(
            match self.status {
                Status::Active => false,
                Status::Completed => true,
            },
            "",
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

        Row::new()
            .push(indentation)
            .push(checkbox)
            .push(summary)
            .push(tags)
            .into()
    }
}
