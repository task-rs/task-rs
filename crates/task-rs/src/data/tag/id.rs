use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[serde(from = "String", into = "String")]
pub struct Id(pub String);

impl<Text: AsRef<str>> From<Text> for Id {
    fn from(tag: Text) -> Self {
        Self(tag.as_ref().to_owned())
    }
}

impl Into<String> for Id {
    fn into(self) -> String {
        self.0
    }
}

#[test]
fn test_from_string_slice() {
    assert_eq!(Id::from("foo"), Id("foo".to_owned()));
}

#[test]
fn test_from_string_object() {
    assert_eq!(Id::from("foo".to_owned()), Id("foo".to_owned()));
}

#[test]
fn test_string_slice_into() {
    let id: Id = "foo".into();
    assert_eq!(id, Id("foo".to_owned()));
}

#[test]
fn test_string_object_into() {
    let id: Id = "foo".to_owned().into();
    assert_eq!(id, Id("foo".to_owned()));
}

#[test]
fn test_into_string_object() {
    let text: String = Id("foo".to_owned()).into();
    assert_eq!(text, "foo".to_owned());
}
