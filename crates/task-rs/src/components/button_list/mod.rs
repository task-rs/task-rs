pub mod controls;

pub use controls::Controls;

use super::super::utils::Callable;
use iced::*;

pub struct ButtonList<'a, Key, Value, Message, ButtonStyle, GetContent, GetMessage, GetButtonStyle>
where
    Key: Ord,
    ButtonStyle: button::StyleSheet,
    GetContent: Callable<Input = &'a Key, Output = Value>,
    GetMessage: Callable<Input = &'a Key, Output = Message>,
    GetButtonStyle: Callable<Input = &'a Key, Output = ButtonStyle>,
{
    pub(crate) controls: &'a mut Controls<Key>,
    pub get_content: GetContent,
    pub get_message: GetMessage,
    pub get_style: GetButtonStyle,
}

macro_rules! impl_into {
    ($container:ident) => {
        impl<'a, Key, Message, ButtonStyle, GetContent, GetMessage, GetButtonStyle>
            Into<$container<'a, Message>>
            for ButtonList<
                'a,
                Key,
                Element<'a, Message>,
                Message,
                ButtonStyle,
                GetContent,
                GetMessage,
                GetButtonStyle,
            >
        where
            Key: Ord + Clone + 'a,
            Message: Clone + 'a,
            ButtonStyle: button::StyleSheet + 'static,
            GetContent: Callable<Input = &'a Key, Output = Element<'a, Message>> + Clone,
            GetMessage: Callable<Input = &'a Key, Output = Message> + Clone,
            GetButtonStyle: Callable<Input = &'a Key, Output = ButtonStyle> + Clone,
        {
            fn into(self) -> $container<'a, Message> {
                let mut container = $container::new();

                for (key, state) in self.controls.0.iter_mut() {
                    let value = self.get_content.clone().call(key);
                    let button = Button::new(state, value)
                        .on_press(self.get_message.clone().call(key))
                        .style(self.get_style.clone().call(key));
                    container = container.push(button);
                }

                container
            }
        }
    };
}

impl_into!(Column);
impl_into!(Row);
