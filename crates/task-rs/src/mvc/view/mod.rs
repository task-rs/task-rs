use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    Row::new()
        .push(Column::new().push(Text::new("TaskRs").color(Color::BLACK).size(40)))
        .into()
}
