use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct Data {
    pub name: Option<String>,
    pub description: Option<String>,
}

fn maybe_text(text: impl Into<String>) -> Option<String> {
    let text = text.into();
    if text.is_empty() {
        None
    } else {
        Some(text)
    }
}

impl Data {
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = maybe_text(name);
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = maybe_text(description);
        self
    }
}

#[test]
fn test_with_name() {
    assert_eq!(
        Data::default().with_name("name"),
        Data {
            name: Some("name".to_owned()),
            description: None,
        },
    );
}

#[test]
fn test_with_description() {
    assert_eq!(
        Data::default().with_description("description"),
        Data {
            name: None,
            description: Some("description".to_owned()),
        },
    );
}

#[test]
fn test_with_name_description() {
    assert_eq!(
        Data::default()
            .with_name("name")
            .with_description("description"),
        Data {
            name: Some("name".to_owned()),
            description: Some("description".to_owned()),
        },
    );
}

#[test]
fn test_erase_name() {
    assert_eq!(
        Data {
            name: Some("name".to_owned()),
            description: Some("description".to_owned()),
        }
        .with_name(""),
        Data {
            name: None,
            description: Some("description".to_owned()),
        },
    )
}

#[test]
fn test_erase_description() {
    assert_eq!(
        Data {
            name: Some("name".to_owned()),
            description: Some("description".to_owned()),
        }
        .with_description(""),
        Data {
            name: Some("name".to_owned()),
            description: None,
        },
    )
}

#[test]
fn test_erase_name_and_description() {
    assert_eq!(
        Data {
            name: Some("name".to_owned()),
            description: Some("description".to_owned()),
        }
        .with_name("")
        .with_description(""),
        Data {
            name: None,
            description: None,
        },
    )
}
