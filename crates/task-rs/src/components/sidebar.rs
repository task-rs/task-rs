use super::super::{data::tag, sizes::sidebar::*, style, utils::Callable};
use super::{
    controls,
    main::model::view::tasks::{FilterMethod, Tasks as TaskView},
    TagFilterMethod, TagList,
};
use iced::*;
use pipe_trait::*;

pub struct Sidebar<'a, Theme> {
    pub tags: &'a tag::Map,
    pub task_view: &'a TaskView,
    pub single_tag: Option<tag::Index>,
    pub theme: Theme,
    pub tag_filter_method_controls: &'a mut controls::TagFilterMethod,
    pub tag_list_controls: &'a mut controls::TagList,
}

impl<'a, Theme> Sidebar<'a, Theme>
where
    Theme: style::Theme + Copy,
{
    pub fn view(self) -> Element<'a, Message> {
        let sidebar = Column::<'a, Message>::new().push(TagFilterMethod {
            controls: self.tag_filter_method_controls,
            filter_method: self.task_view.filter_method,
            theme: self.theme,
            all_message: Message::SetTaskFilterMethodToAll,
            single_tag_message: Message::SetTaskFilterMethodToSingleTag,
            multiple_tags_message: Message::SetTaskFilterMethodToMultipleTags,
            check_all_tags: Message::CheckAllOfMultipleTags,
            uncheck_all_tags: Message::UncheckAllOfMultipleTags,
            invert_all_tags: Message::InvertAllOfMultipleTags,
            enable_check_all: self.task_view.multiple_tags.len() != self.tags.len(),
            enable_uncheck_all: !self.task_view.multiple_tags.is_empty(),
        });

        let tag_list = TagList {
            controls: self.tag_list_controls,
            button_prefix: match self.task_view.filter_method {
                FilterMethod::All | FilterMethod::SingleTag => "‣",
                FilterMethod::MultipleTags => "✓",
            },
            get_content: GetContent { map: self.tags },
            get_activated: GetActivated {
                task_view: self.task_view,
                tags: self.tags,
            },
            get_message: GetMessage {
                task_view: self.task_view,
                tags: self.tags,
            },
            theme: self.theme,
        };

        sidebar.push(tag_list).into()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTaskFilterMethodToAll,
    SetTaskFilterMethodToSingleTag,
    SetTaskFilterMethodToMultipleTags,
    FilterTaskBySingleTag(tag::Index),
    AddTagToMultipleTags(tag::Index),
    RemoveTagFromMultipleTags(tag::Index),
    CheckAllOfMultipleTags,
    UncheckAllOfMultipleTags,
    InvertAllOfMultipleTags,
}

#[derive(Debug, Copy, Clone)]
struct GetContent<'a> {
    map: &'a tag::Map,
}
impl<'a> Callable for GetContent<'a> {
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
        .pipe(Container::new)
        .width(Length::Units(TAG_CONTENT_LENGTH))
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
struct GetMessage<'a> {
    task_view: &'a TaskView,
    tags: &'a tag::Map,
}
impl<'a> Callable for GetMessage<'a> {
    type Input = tag::Index;
    type Output = Message;
    fn call(self, x: Self::Input) -> Self::Output {
        match self.task_view.filter_method {
            FilterMethod::All | FilterMethod::SingleTag => Message::FilterTaskBySingleTag(x),
            FilterMethod::MultipleTags => {
                if let Some(id) = self.tags.get_key_by_index(x) {
                    if self.task_view.multiple_tags.contains(id.as_ref()) {
                        Message::RemoveTagFromMultipleTags(x)
                    } else {
                        Message::AddTagToMultipleTags(x)
                    }
                } else {
                    panic!("Invalid State: tag of {:?} does not exist", x)
                }
            }
        }
    }
}
