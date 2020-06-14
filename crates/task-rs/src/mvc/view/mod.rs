use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let color_scheme = &model.ui_state.theme.color_scheme();

    Row::new()
        .push(Column::new().push(Text::new("TaskRs").color(color_scheme.text).size(40)))
        .into()
}
