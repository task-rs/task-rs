pub mod controls;

pub use controls::Controls;

use iced::*;

pub struct ButtonList<'a, Key, Value, Message, ButtonStyle, GetButtonStyle>
where
    Key: Ord,
    ButtonStyle: button::StyleSheet,
    GetButtonStyle: Fn(&Key) -> ButtonStyle,
{
    pub(crate) controls: &'a mut Controls<Key>,
    pub get_content: fn(&Key) -> Value,
    pub get_message: fn(&Key) -> Message,
    pub get_style: GetButtonStyle,
}

macro_rules! impl_into {
    ($container:ident) => {
        impl<'a, Key, Message, ButtonStyle, GetButtonStyle> Into<$container<'a, Message>>
            for ButtonList<'a, Key, Element<'a, Message>, Message, ButtonStyle, GetButtonStyle>
        where
            Key: Ord + Clone + 'a,
            Message: Clone + 'a,
            ButtonStyle: button::StyleSheet + 'static,
            GetButtonStyle: Fn(&Key) -> ButtonStyle,
        {
            fn into(self) -> $container<'a, Message> {
                let mut container = $container::new();

                for (key, state) in self.controls.0.iter_mut() {
                    let value = (self.get_content)(key);
                    let button = Button::new(state, value)
                        .on_press((self.get_message)(key))
                        .style((self.get_style)(key));
                    container = container.push(button);
                }

                container
            }
        }
    };
}

impl_into!(Column);
impl_into!(Row);
