use super::super::super::data::tag;
use super::super::{Message, Model};
use iced::*;
use pipe_trait::*;

pub fn create(model: &Model) -> Element<'_, Message> {
    let mut sidebar = Column::new();

    sidebar = "All".pipe(Text::new).pipe(|text| sidebar.push(text));

    for entry in &model.data.tags {
        sidebar = entry
            .pipe(tag::entry::display)
            .pipe(Text::new)
            .pipe(|text| sidebar.push(text))
    }

    sidebar.into()
}
