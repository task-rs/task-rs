use super::{Message, Model};
use iced::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let style = model.ui_state.theme.style();

    let row = Row::new().push(Column::new().push(Text::new("TaskRs").size(40)));

    Container::new(row)
        .style(stylesheets::Container(style))
        .into()
}

mod stylesheets;
