use super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn view(model: &mut Model) -> Element<'_, Message> {
    let style = model.ui_state.theme.style();

    Column::new()
        .push(header::create(&mut model.controls))
        .push(Row::new().push(sidebar::create(model.data.tags.iter())))
        .pipe(Container::new)
        .style(stylesheets::Container(style))
        .into()
}

mod header;
mod sidebar;
mod stylesheets;
