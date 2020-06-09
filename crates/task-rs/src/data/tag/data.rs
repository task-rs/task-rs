use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
    pub name: Option<String>,
    pub description: Option<String>,
}
