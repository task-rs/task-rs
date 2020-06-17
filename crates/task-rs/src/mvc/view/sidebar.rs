use super::super::super::data::tag;
use super::super::Message;
use iced::*;
use pipe_trait::*;

pub fn create<'a>(
    entries: impl IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
) -> Element<'a, Message> {
    let mut sidebar = Column::new();

    sidebar = "All".pipe(Text::new).pipe(|text| sidebar.push(text));

    for entry in entries {
        sidebar = entry
            .pipe(tag::entry::display)
            .pipe(Text::new)
            .pipe(|text| sidebar.push(text))
    }

    sidebar.into()
}
