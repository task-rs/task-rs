use super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let style = model.ui_state.theme.style();

    Column::new()
        .push(header::create(model))
        .push(Row::new().push(sidebar::create(model)))
        .pipe(Container::new)
        .style(stylesheets::Container(style))
        .into()
}

mod header;
mod sidebar;
mod stylesheets;
