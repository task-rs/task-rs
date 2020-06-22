use super::super::{
    data::tag,
    mvc::model::view::tasks::{FilterMethod, Tasks as TaskView},
    style,
    utils::Callable,
};
use super::{controls, TagFilterMethod, TagList};
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

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
    pub set_task_filter_method_to_single_tag: Message,
    pub set_task_filter_method_to_multiple_tags: Message,
    pub filter_tasks_by_single_tag: fn(&tag::Id) -> Message,
    pub add_tag_to_multiple_tags: fn(tag::Id) -> Message,
    pub remove_tag_from_multiple_tags: fn(tag::Id) -> Message,
    pub(crate) tag_filter_method_controls: &'a mut controls::TagFilterMethod,
    pub(crate) tag_list_controls: &'a mut controls::TagList,
}

impl<'a, Tags, Theme, Message> Into<Element<'a, Message>> for Sidebar<'a, Tags, Theme, Message>
where
    Tags: IntoIterator<Item = (&'a tag::Id, &'a tag::Data)>,
    Theme: style::Theme + Copy,
    Message: Clone + 'static,
{
    fn into(self) -> Element<'a, Message> {
        let sidebar = Column::<'a, Message>::new().push(TagFilterMethod {
            controls: self.tag_filter_method_controls,
            filter_method: self.task_view.filter_method,
            theme: self.theme,
            all_message: self.set_task_filter_method_to_all,
            single_tag_message: self.set_task_filter_method_to_single_tag,
            multiple_tags_message: self.set_task_filter_method_to_multiple_tags,
        });

        *self.tag_list_controls = self
            .tags
            .into_iter()
            .map(|(id, _)| (id.clone(), button::State::default()))
            .collect::<BTreeMap<_, _>>()
            .pipe(controls::TagList::new);

        let tag_list = TagList {
            controls: self.tag_list_controls,
            get_content: |id| id.0.pipe_ref(Text::new).into(),
            get_activated: GetActivated(self.task_view),
            get_message: |id| match self.task_view.filter_method {
                FilterMethod::All | FilterMethod::SingleTag => {
                    (self.filter_tasks_by_single_tag)(id)
                }
                FilterMethod::MultipleTags => (self.add_tag_to_multiple_tags)(id),
            },
            theme: self.theme,
        };

        sidebar.push(tag_list).into()
    }
}

struct GetActivated<'a>(&'a TaskView);
impl<'a> Callable for GetActivated<'a> {
    type Input = &'a tag::Id;
    type Output = bool;
    fn call(self, id: &tag::Id) -> bool {
        match self.0.filter_method {
            FilterMethod::All => false,
            FilterMethod::SingleTag => &self.0.single_tag == id,
            FilterMethod::MultipleTags => self.0.multiple_tags.contains(id),
        }
    }
}
