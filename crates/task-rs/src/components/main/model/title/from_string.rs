use super::Title;

impl<Text: AsRef<str>> From<Text> for Title {
    fn from(text: Text) -> Self {
        Title(text.as_ref().to_owned())
    }
}

#[test]
fn test_from_string() {
    assert_eq!(Title::from("foo".to_owned()), Title("foo".to_owned()));
}

#[test]
fn test_from_str() {
    assert_eq!(Title::from("foo"), Title("foo".to_owned()));
}
