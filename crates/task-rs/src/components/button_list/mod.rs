pub mod controls;

pub use controls::Controls;

use super::super::style;
use iced::*;

pub struct ButtonList<'a, Key, Value, Theme, Message>
where
    Theme: style::Theme,
    Key: Ord,
{
    pub(crate) controls: &'a mut Controls<Key>,
    pub get_content: fn(&Key) -> Value,
    pub get_message: fn(&Key) -> Message,
    pub theme: Theme,
}

macro_rules! impl_into {
    ($container:ident) => {
        impl<'a, Key, Value, Theme, Message> Into<$container<'a, Message>>
            for ButtonList<'a, Key, Value, Theme, Message>
        where
            Key: Ord + Clone + 'a,
            Value: Into<Element<'a, Message>>,
            Theme: style::Theme,
            Message: Clone + 'a,
        {
            fn into(self) -> $container<'a, Message> {
                let mut container = $container::new();

                for (key, state) in self.controls.0.iter_mut() {
                    let value = (self.get_content)(key);
                    let button = Button::new(state, value);
                    container = container.push(button);
                }

                container
            }
        }
    };
}

impl_into!(Column);
impl_into!(Row);
