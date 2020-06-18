pub use super::task_status_filter::Controls as TaskStatusFilter;

#[derive(Debug, Default, Clone)]
pub struct Controls {
    pub(crate) task_status_filter: TaskStatusFilter,
}
