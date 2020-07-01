use super::Title;

impl ToString for Title {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}

#[test]
fn test_to_string() {
    assert_eq!(Title("foo".to_owned()).to_string(), "foo");
}
