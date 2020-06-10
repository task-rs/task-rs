use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "String", into = "String")]
pub struct Title(String);

mod default;
mod from_string;
mod into_string;
mod to_string;
