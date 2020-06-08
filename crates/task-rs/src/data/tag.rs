use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
pub struct TagId(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TagData {
    pub description: Option<String>,
}
