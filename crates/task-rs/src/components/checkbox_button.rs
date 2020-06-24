use super::IndentedButton;
use iced::*;

pub struct CheckboxButton<'a, Message> {
    pub state: &'a mut button::State,
    pub checked: bool,
    pub content: Element<'a, Message>,
}

impl<'a, Message> CheckboxButton<'a, Message>
where
    Message: 'a,
{
    pub fn into_indented_button(self) -> IndentedButton<'a, &'a str, Message> {
        self.into()
    }

    pub fn into_button(self) -> Button<'a, Message> {
        self.into()
    }
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
        self.into_indented_button().into()
    }
}
