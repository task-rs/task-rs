use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "String", into = "String")]
pub struct Title(String);

impl ToString for Title {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

impl From<String> for Title {
    fn from(text: String) -> Self {
        Title(text)
    }
}

impl Into<String> for Title {
    fn into(self) -> String {
        self.0
    }
}

impl Default for Title {
    fn default() -> Self {
        "TaskRs".to_owned().into()
    }
}
