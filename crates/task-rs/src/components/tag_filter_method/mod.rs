pub mod controls;

pub use controls::Controls;

pub use super::super::mvc::model::view::tasks::FilterMethod as Value;

use super::super::style;
use super::theme;
use iced::*;
use pipe_trait::*;

pub struct TagFilterMethod<'a, Theme, Message>
where
    Theme: theme::Theme,
{
    pub(crate) controls: &'a mut Controls,
    pub filter_method: Value,
    pub all_message: Message,
    pub theme: Theme,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for TagFilterMethod<'a, Theme, Message>
where
    Message: Clone + 'a,
    Theme: theme::Theme + Copy,
{
    fn into(self) -> Element<'a, Message> {
        match self.filter_method {
            Value::All => Button::new(&mut self.controls.0, Text::new("All"))
                .style(AllButtonStyleSheet {
                    style: self.theme.style(),
                    activated: true,
                })
                .into(),
            Value::SingleTag => Button::new(&mut self.controls.0, Text::new("All"))
                .style(AllButtonStyleSheet {
                    style: self.theme.style(),
                    activated: false,
                })
                .on_press(self.all_message)
                .into(),
            Value::MultipleTags => {
                Button::new(&mut self.controls.0, Text::new("Select All")).into()
            }
        }
    }
}

struct AllButtonStyleSheet {
    pub style: style::Style,
    pub activated: bool,
}

impl button::StyleSheet for AllButtonStyleSheet {
    fn active(&self) -> button::Style {
        let AllButtonStyleSheet { style, activated } = self;

        button::Style {
            background: if *activated {
                style.activated_button_bg
            } else {
                style.inactivated_button_bg
            }
            .pipe(Background::Color)
            .pipe(Some),
            text_color: if *activated {
                style.activated_button_text
            } else {
                style.inactivated_button_text
            },
            ..button::Style::default()
        }
    }
}
