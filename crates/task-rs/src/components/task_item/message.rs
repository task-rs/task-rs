use super::super::super::data::Status;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    SetStatus(Vec<usize>, Status),
}
