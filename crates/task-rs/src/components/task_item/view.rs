use super::super::super::{data::Status, sizes::main::SUB_TASK_INDENT};
use super::{Message, TaskItem};
use iced::*;
use pipe_trait::*;

impl TaskItem {
    pub fn view(self) -> Element<'static, Message> {
        let TaskItem {
            tags,
            summary,
            address,
            ..
        } = self;

        let indent_size = address
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
            "",
            move |is_checked| {
                Message::SetStatus(
                    address.as_ref().clone(),
                    if is_checked {
                        Status::Completed
                    } else {
                        Status::Active
                    },
                )
            },
        );

        let middle = {
            let summary = Text::new(summary);

            let tags = {
                let mut row = Row::new();

                for tag in &tags {
                    let label = format!("#{}", &tag.0);
                    let label = Text::new(label);
                    row = row.push(label);
                }

                row
            };

            Column::new().push(summary).push(tags)
        };

        Row::new()
            .push(indentation)
            .push(checkbox)
            .push(middle)
            .into()
    }
}
