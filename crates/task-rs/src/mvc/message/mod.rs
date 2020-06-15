#[derive(Debug, Copy, Clone)]
pub enum Message {
    TaskStatusFilterActive(bool),
    TaskStatusFilterCompleted(bool),
}
