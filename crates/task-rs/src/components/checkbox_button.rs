use iced::*;
use pipe_trait::*;

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
        let prefix = if self.checked { "✓" } else { "" }
            .pipe(Text::new)
            .pipe(Container::new)
            .width(Length::Units(16));
        let content = Row::new().push(prefix).push(self.content);
        Button::new(self.state, content)
    }
}
