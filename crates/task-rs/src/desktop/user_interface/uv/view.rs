use super::super::{Message, UserInterface};
use iced::*;

pub fn view(user_interface: &mut UserInterface) -> Element<'_, Message> {
    Row::new()
        .push(Column::new().push(Text::new("TaskRs").color(Color::BLACK).size(40)))
        .into()
}
