use super::Title;

impl From<String> for Title {
    fn from(text: String) -> Self {
        Title(text)
    }
}
