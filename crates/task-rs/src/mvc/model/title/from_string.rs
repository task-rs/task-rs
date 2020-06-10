use super::Title;

impl From<String> for Title {
    fn from(text: String) -> Self {
        Title(text)
    }
}

#[test]
fn test_from_string() {
    assert_eq!(Title::from("foo".to_owned()), Title("foo".to_owned()));
}
