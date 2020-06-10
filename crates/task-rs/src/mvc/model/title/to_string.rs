use super::Title;

impl ToString for Title {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}
