use super::super::{data::tag, mvc::model::view::tasks::Tasks as TaskView, style};
use super::{controls, TagFilterMethod};
use iced::*;
use pipe_trait::*;

pub struct Sidebar<'a, Tags, Theme, Message>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Theme: style::Theme,
{
    pub tags: Tags,
    pub task_view: &'a TaskView,
    pub theme: Theme,
    pub set_task_filter_method_to_all: Message,
    pub(crate) tag_filter_method_controls: &'a mut controls::TagFilterMethod,
}

impl<'a, Tags, Theme, Message> Into<Element<'a, Message>> for Sidebar<'a, Tags, Theme, Message>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Theme: style::Theme + Copy,
    Message: Clone + 'a,
{
    fn into(self) -> Element<'a, Message> {
        let mut sidebar = Column::<'a, Message>::new()
            .push(
                Row::new()
                    .push(Text::new("select"))
                    .push(Text::new("filter")),
            )
            .push(TagFilterMethod {
                controls: self.tag_filter_method_controls,
                filter_method: self.task_view.filter_method,
                theme: self.theme,
                all_message: self.set_task_filter_method_to_all,
            });

        for entry in self.tags {
            sidebar = entry
                .pipe(tag::entry::display)
                .pipe(Text::new)
                .pipe(|text| sidebar.push(text))
        }

        sidebar.into()
    }
}
