use super::super::data::tag;
use iced::*;
use pipe_trait::*;

pub struct Sidebar<'a, Tags>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
{
    pub tags: Tags,
}

impl<'a, Tags, Message> Into<Element<'a, Message>> for Sidebar<'a, Tags>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Message: 'a,
{
    fn into(self) -> Element<'a, Message> {
        let mut sidebar = Column::new();

        sidebar = "All".pipe(Text::new).pipe(|text| sidebar.push(text));

        for entry in self.tags {
            sidebar = entry
                .pipe(tag::entry::display)
                .pipe(Text::new)
                .pipe(|text| sidebar.push(text))
        }

        sidebar.into()
    }
}
