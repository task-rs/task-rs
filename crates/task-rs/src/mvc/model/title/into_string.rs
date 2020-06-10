use super::Title;

impl Into<String> for Title {
    fn into(self) -> String {
        self.0
    }
}
