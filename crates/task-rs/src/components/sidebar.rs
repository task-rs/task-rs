use super::super::{
    data::tag,
    mvc::model::view::tasks::{FilterMethod, Tasks as TaskView},
    style,
    utils::Callable,
};
use super::{controls, TagFilterMethod, TagList};
use core::marker::PhantomData;
use iced::*;
use pipe_trait::*;

pub struct Sidebar<'a, Theme, Message> {
    pub tags: &'a tag::Map,
    pub task_view: &'a TaskView,
    pub single_tag: Option<tag::Id>,
    pub theme: Theme,
    pub set_task_filter_method_to_all: Message,
    pub set_task_filter_method_to_single_tag: Message,
    pub set_task_filter_method_to_multiple_tags: Message,
    pub filter_tasks_by_single_tag: fn(tag::Index) -> Message,
    pub add_tag_to_multiple_tags: fn(tag::Index) -> Message,
    pub remove_tag_from_multiple_tags: fn(tag::Index) -> Message,
    pub check_all_of_multiple_tags: Message,
    pub uncheck_all_of_multiple_tags: Message,
    pub invert_all_of_multiple_tags: Message,
    pub(crate) tag_filter_method_controls: &'a mut controls::TagFilterMethod,
    pub(crate) tag_list_controls: &'a mut controls::TagList,
}

impl<'a, Theme, Message> Into<Element<'a, Message>> for Sidebar<'a, Theme, Message>
where
    Theme: style::Theme + Copy,
    Message: Clone + 'a,
{
    fn into(self) -> Element<'a, Message> {
        let sidebar = Column::<'a, Message>::new().push(TagFilterMethod {
            controls: self.tag_filter_method_controls,
            filter_method: self.task_view.filter_method,
            theme: self.theme,
            all_message: self.set_task_filter_method_to_all,
            single_tag_message: self.set_task_filter_method_to_single_tag,
            multiple_tags_message: self.set_task_filter_method_to_multiple_tags,
            check_all_tags: self.check_all_of_multiple_tags,
            uncheck_all_tags: self.uncheck_all_of_multiple_tags,
            invert_all_tags: self.invert_all_of_multiple_tags,
        });

        let tag_list = TagList {
            controls: self.tag_list_controls,
            button_prefix: "✓",
            get_content: GetContent {
                map: self.tags,
                _phantom_msg: PhantomData,
            },
            get_activated: GetActivated {
                task_view: self.task_view,
                tags: self.tags,
            },
            get_message: GetMessage {
                task_view: self.task_view,
                tags: self.tags,
                filter_tasks_by_single_tag: self.filter_tasks_by_single_tag,
                add_tag_to_multiple_tags: self.add_tag_to_multiple_tags,
                remove_tag_from_multiple_tags: self.remove_tag_from_multiple_tags,
            },
            theme: self.theme,
        };

        sidebar.push(tag_list).into()
    }
}

#[derive(Debug, Copy, Clone)]
struct GetContent<'a, Message> {
    map: &'a tag::Map,
    _phantom_msg: PhantomData<Message>,
}
impl<'a, Message> Callable for GetContent<'a, Message> {
    type Input = tag::Index;
    type Output = Element<'a, Message>;
    fn call(self, x: Self::Input) -> Self::Output {
        let id = self
            .map
            .get_key_by_index(x)
            .unwrap_or_else(|| panic!("Invalid State: tag of {:?} does not exist", x))
            .as_ref();
        if let Some(data) = self.map.get_value_by_index(x) {
            tag::entry::display((id, data))
        } else {
            id.0.clone()
        }
        .pipe(Text::new)
        .into()
    }
}

#[derive(Debug, Copy, Clone)]
struct GetActivated<'a> {
    task_view: &'a TaskView,
    tags: &'a tag::Map,
}
impl<'a> Callable for GetActivated<'a> {
    type Input = tag::Index;
    type Output = bool;
    fn call(self, x: Self::Input) -> Self::Output {
        match self.task_view.filter_method {
            FilterMethod::All => false,
            FilterMethod::SingleTag => {
                if let Some(current_id) = &self.task_view.single_tag {
                    Some(current_id) == self.tags.get_key_by_index(x).map(AsRef::as_ref)
                } else {
                    false
                }
            }
            FilterMethod::MultipleTags => {
                if let Some(id) = self.tags.get_key_by_index(x) {
                    self.task_view.multiple_tags.contains(id.as_ref())
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
struct GetMessage<'a, Message> {
    task_view: &'a TaskView,
    tags: &'a tag::Map,
    filter_tasks_by_single_tag: fn(tag::Index) -> Message,
    add_tag_to_multiple_tags: fn(tag::Index) -> Message,
    remove_tag_from_multiple_tags: fn(tag::Index) -> Message,
}
impl<'a, Message> Callable for GetMessage<'a, Message> {
    type Input = tag::Index;
    type Output = Message;
    fn call(self, x: Self::Input) -> Self::Output {
        match self.task_view.filter_method {
            FilterMethod::All | FilterMethod::SingleTag => (self.filter_tasks_by_single_tag)(x),
            FilterMethod::MultipleTags => {
                if let Some(id) = self.tags.get_key_by_index(x) {
                    (if self.task_view.multiple_tags.contains(id.as_ref()) {
                        self.remove_tag_from_multiple_tags
                    } else {
                        self.add_tag_to_multiple_tags
                    })(x)
                } else {
                    panic!("Invalid State: tag of {:?} does not exist", x)
                }
            }
        }
    }
}
