pub mod controls;

pub use controls::Controls;

pub use super::main::model::view::tasks::FilterMethod as Value;

use super::super::{sizes::sidebar::*, style, utils::on_press_if};
use super::IndentedButton;
use iced::*;
use pipe_trait::*;

pub struct TagFilterMethod<'a, Theme>
where
    Theme: style::Theme,
{
    pub controls: &'a mut Controls,
    pub filter_method: Value,
    pub enable_check_all: bool,
    pub enable_uncheck_all: bool,
    pub theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    All,
    SingleTag,
    MultipleTags,
    CheckAll,
    UncheckAll,
    InvertAll,
}

impl<'a, Theme> TagFilterMethod<'a, Theme>
where
    Theme: style::Theme + Copy,
{
    pub fn view(self) -> Element<'a, Message> {
        let Controls {
            ref mut filter_method_single_tag,
            ref mut filter_method_multiple_tags,
            ref mut all_button,
            mass_check_button: (ref mut check_all, ref mut uncheck_all, ref mut invert_all),
        } = self.controls;

        let centered_text = |text: &str| {
            text.pipe(Text::new)
                .pipe(Container::new)
                .width(Length::Fill)
                .center_x()
        };

        macro_rules! all_button {
            ($activated:expr, $prefix:expr) => {
                IndentedButton {
                    state: all_button,
                    prefix: $prefix,
                    content: Text::new("All")
                        .pipe(Container::new)
                        .width(Length::Fill)
                        .into(),
                }
                .into_button()
                .width(Length::Units(SIDEBAR_LENGTH))
                .on_press(Message::All)
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
                        Button::new(filter_method_single_tag, centered_text("select"))
                            .width(Length::Units(FILTER_METHOD_BUTTON_LENGTH))
                            .on_press(Message::SingleTag)
                            .style(style::BinaryStateButton {
                                style: self.theme.style(),
                                activated: self.filter_method != Value::MultipleTags,
                            }),
                    )
                    .push(
                        Button::new(filter_method_multiple_tags, centered_text("filter"))
                            .width(Length::Units(FILTER_METHOD_BUTTON_LENGTH))
                            .on_press(Message::MultipleTags)
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
                        Button::new(check_all, centered_text("all"))
                            .width(Length::Units(MASS_CHECK_BUTTON_LENGTH))
                            .pipe(on_press_if(self.enable_check_all, Message::CheckAll))
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .push(
                        Button::new(uncheck_all, centered_text("none"))
                            .width(Length::Units(MASS_CHECK_BUTTON_LENGTH))
                            .pipe(on_press_if(self.enable_uncheck_all, Message::UncheckAll))
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .push(
                        Button::new(invert_all, centered_text("invert"))
                            .width(Length::Units(MASS_CHECK_BUTTON_LENGTH))
                            .on_press(Message::InvertAll)
                            .style(style::SingleStateButton(self.theme.style())),
                    )
                    .into(),
            })
            .into()
    }
}
