use iced::*;

pub struct CheckboxButton<'a, Message> {
    pub checked: bool,
    pub content: Element<'a, Message>,
}

impl<'a, Message> CheckboxButton<'a, Message> {
    pub fn from_widget(checked: bool, content: impl Into<Element<'a, Message>>) -> Self {
        CheckboxButton {
            checked,
            content: content.into(),
        }
    }

    pub fn from_text(checked: bool, content: impl Into<String>) -> Self {
        Self::from_widget(checked, Text::new(content))
    }

    pub fn into_button(self, state: &'a mut button::State) -> Button<Message>
    where
        Message: 'a,
    {
        let content = Row::new()
            .push(Text::new(if self.checked { "✓" } else { " " }))
            .push(self.content);
        Button::new(state, content)
    }
}
