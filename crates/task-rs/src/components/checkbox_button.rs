use iced::*;

pub struct CheckboxButton<'a, Message> {
    pub state: &'a mut button::State,
    pub checked: bool,
    pub content: Element<'a, Message>,
}

impl<'a, Message> Into<Button<'a, Message>> for CheckboxButton<'a, Message>
where
    Message: 'a,
{
    fn into(self) -> Button<'a, Message> {
        let content = Row::new()
            .push(Text::new(if self.checked { "✓" } else { " " }))
            .push(self.content);
        Button::new(self.state, content)
    }
}
