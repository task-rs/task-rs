pub mod controls;

pub use controls::Controls;

pub use super::super::mvc::model::view::tasks::FilterMethod as Value;

use super::super::style;
use iced::*;

pub struct TagFilterMethod<'a, Theme, Message>
where
    Theme: style::Theme,
{
    pub(crate) controls: &'a mut Controls,
    pub filter_method: Value,
    pub all_message: Message,
    pub single_tag_message: Message,
    pub multiple_tags_message: Message,
    pub theme: Theme,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for TagFilterMethod<'a, Theme, Message>
where
    Message: Clone + 'a,
    Theme: style::Theme + Copy,
{
    fn into(self) -> Element<'a, Message> {
        let Controls {
            ref mut filter_method_single_tag,
            ref mut filter_method_multiple_tags,
            ref mut all_button,
        } = self.controls;

        Column::new()
            .push(
                Row::new()
                    .push(
                        Button::new(filter_method_single_tag, Text::new("select"))
                            .on_press(self.single_tag_message)
                            .style(style::BinaryStateButton {
                                style: self.theme.style(),
                                activated: self.filter_method != Value::MultipleTags,
                            }),
                    )
                    .push(
                        Button::new(filter_method_multiple_tags, Text::new("filter"))
                            .on_press(self.multiple_tags_message)
                            .style(style::BinaryStateButton {
                                style: self.theme.style(),
                                activated: self.filter_method == Value::MultipleTags,
                            }),
                    ),
            )
            .push::<Element<'a, Message>>(match self.filter_method {
                Value::All => Button::new(all_button, Text::new("All"))
                    .style(style::BinaryStateButton {
                        style: self.theme.style(),
                        activated: true,
                    })
                    .into(),
                Value::SingleTag => Button::new(all_button, Text::new("All"))
                    .style(style::BinaryStateButton {
                        style: self.theme.style(),
                        activated: false,
                    })
                    .on_press(self.all_message)
                    .into(),
                Value::MultipleTags => Button::new(all_button, Text::new("Select All")).into(),
            })
            .into()
    }
}
