use super::IndentedButton;
use iced::*;
use pipe_trait::*;

pub struct CheckboxButton<'a, Message> {
    pub state: &'a mut button::State,
    pub checked: bool,
    pub content: Element<'a, Message>,
}

impl<'a, Message> Into<IndentedButton<'a, &'a str, Message>> for CheckboxButton<'a, Message>
where
    Message: 'a,
{
    fn into(self) -> IndentedButton<'a, &'a str, Message> {
        IndentedButton {
            state: self.state,
            content: self.content,
            prefix: if self.checked { "✓" } else { "" },
        }
    }
}

impl<'a, Message> Into<Button<'a, Message>> for CheckboxButton<'a, Message>
where
    Message: 'a,
{
    fn into(self) -> Button<'a, Message> {
        self.pipe(Into::<IndentedButton<'a, _, _>>::into).into()
    }
}
