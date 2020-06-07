use super::Reason;

#[derive(Debug)]
pub struct Error {
    pub text: String,
    pub reason: Reason,
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!(
            "{:?} is invalid. reason: {}",
            self.text,
            self.reason.to_string()
        )
    }
}
