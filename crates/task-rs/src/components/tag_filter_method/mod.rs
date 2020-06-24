pub mod controls;

pub use controls::Controls;

pub use super::super::mvc::model::view::tasks::FilterMethod as Value;

use super::super::style;
use super::IndentedButton;
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
    pub check_all_tags: Message,
    pub uncheck_all_tags: Message,
    pub invert_all_tags: Message,
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
            mass_check_button: (ref mut check_all, ref mut uncheck_all, ref mut invert_all),
        } = self.controls;

        macro_rules! all_button {
            ($activated:expr, $prefix:expr) => {
                IndentedButton {
                    state: all_button,
                    prefix: $prefix,
                    content: Text::new("All").into(),
                }
                .into_button()
                .on_press(self.all_message)
                .style(style::BinaryStateButton {
                    style: self.theme.style(),
                    activated: $activated,
                })
                .into()
            };
        }

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
                Value::All => all_button!(true, "‣"),
                Value::SingleTag => all_button!(false, ""),
                Value::MultipleTags => Row::new()
                    .push(
                        Button::new(check_all, Text::new("all"))
                            .on_press(self.check_all_tags)
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .push(
                        Button::new(uncheck_all, Text::new("none"))
                            .on_press(self.uncheck_all_tags)
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .push(
                        Button::new(invert_all, Text::new("invert"))
                            .on_press(self.invert_all_tags)
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .into(),
            })
            .into()
    }
}
