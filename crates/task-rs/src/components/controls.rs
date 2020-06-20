pub use super::tag_filter_method::Controls as TagFilterMethod;
pub use super::task_status_filter::Controls as TaskStatusFilter;
pub use super::theme_switcher::Controls as ThemeSwitcher;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub(crate) task_status_filter: TaskStatusFilter,
    pub(crate) theme_switcher: ThemeSwitcher,
    pub(crate) tag_filter_method: TagFilterMethod,
}
