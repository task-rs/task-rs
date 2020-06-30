pub use super::tag_filter_method::Controls as TagFilterMethod;
pub use super::tag_list::Controls as TagList;
pub use super::task_status_filter::Controls as TaskStatusFilter;
pub use super::theme_switcher::Controls as ThemeSwitcher;
pub use super::{TaskItem, TaskList};

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub(crate) task_status_filter: TaskStatusFilter,
    pub(crate) theme_switcher: ThemeSwitcher,
    pub(crate) tag_filter_method: TagFilterMethod,
    pub(crate) tag_list: TagList,
    pub(crate) task_list: TaskList,
}
