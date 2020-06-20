use super::super::{
    data::tag,
    mvc::model::view::tasks::{FilterMethod, Tasks as TaskView},
    style,
};
use super::{controls, TagFilterMethod};
use iced::*;

pub struct Sidebar<'a, Tags, Theme, Message>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Theme: style::Theme,
{
    pub tags: Tags,
    pub task_view: &'a TaskView,
    pub single_tag: tag::Id,
    pub theme: Theme,
    pub set_task_filter_method_to_all: Message,
    pub filter_tasks_by_single_tag: fn(&tag::Id) -> Message,
    pub add_tag_to_multiple_tags: fn(tag::Id) -> Message,
    pub remove_tag_from_multiple_tags: fn(tag::Id) -> Message,
    pub(crate) tag_filter_method_controls: &'a mut controls::TagFilterMethod,
}

impl<'a, Tags, Theme, Message> Into<Element<'a, Message>> for Sidebar<'a, Tags, Theme, Message>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Theme: style::Theme + Copy,
    Message: Clone + 'static,
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
            let (id, _) = entry;
            let label = tag::entry::display(entry);
            let tag_button: Element<'a, Message> =
                if self.task_view.filter_method == FilterMethod::MultipleTags {
                    let is_checked = self.task_view.multiple_tags.contains(id);
                    let add = self.add_tag_to_multiple_tags;
                    let remove = self.remove_tag_from_multiple_tags;
                    let id = id.clone();
                    Checkbox::new(is_checked, label, move |checked| {
                        if checked {
                            add(id.clone())
                        } else {
                            remove(id.clone())
                        }
                    })
                    .into()
                } else {
                    let selected = if self.task_view.filter_method == FilterMethod::SingleTag {
                        Some(&self.single_tag)
                    } else {
                        None
                    };
                    Radio::new(id, label, selected, self.filter_tasks_by_single_tag).into()
                };
            sidebar = sidebar.push(tag_button);
        }

        sidebar.into()
    }
}
