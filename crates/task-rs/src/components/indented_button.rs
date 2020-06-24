use super::super::sizes::indent_square;
use iced::*;
use pipe_trait::*;

pub struct IndentedButton<'a, Prefix, Message> {
    pub state: &'a mut button::State,
    pub prefix: Prefix,
    pub content: Element<'a, Message>,
}

impl<'a, Prefix, Message> IndentedButton<'a, Prefix, Message> {
    pub fn into_button(self) -> Button<'a, Message>
    where
        Message: 'a,
        Prefix: Into<String>,
    {
        self.into()
    }
}

impl<'a, Prefix, Message> Into<Button<'a, Message>> for IndentedButton<'a, Prefix, Message>
where
    Message: 'a,
    Prefix: Into<String>,
{
    fn into(self) -> Button<'a, Message> {
        let prefix = self
            .prefix
            .pipe(Text::new)
            .pipe(Container::new)
            .width(Length::Units(indent_square::SIZE));
        let content = Row::new().push(prefix).push(self.content);
        Button::new(self.state, content)
    }
}
