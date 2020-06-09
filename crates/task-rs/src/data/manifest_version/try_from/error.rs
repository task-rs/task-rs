use super::super::{MANIFEST_VERSION, N};
use core::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Eq, PartialEq)]
pub struct Error(pub N);

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Expecting manifest version to be {}, but received {}",
            MANIFEST_VERSION, self.0
        )
    }
}

#[test]
fn test_display() {
    assert_eq!(
        format!("{}", Error(1234)),
        "Expecting manifest version to be 0, but received 1234",
    );
}
