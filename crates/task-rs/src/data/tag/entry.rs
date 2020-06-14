use super::{Data, Id};

pub fn display((id, data): (&Id, &Data)) -> String {
    data.name.clone().unwrap_or_else(|| id.0.clone())
}
