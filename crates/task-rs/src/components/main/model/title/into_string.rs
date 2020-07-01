use super::Title;

impl Into<String> for Title {
    fn into(self) -> String {
        self.0
    }
}

#[test]
fn test_into_string() {
    let text: String = Title("foo".to_owned()).into();
    assert_eq!(text, "foo");
}
