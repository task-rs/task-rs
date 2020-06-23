#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index(u32);

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index(value)
    }
}

#[test]
fn test_from_num() {
    assert_eq!(Index::from(123), Index(123));
}

impl Into<u32> for Index {
    fn into(self) -> u32 {
        self.0
    }
}

#[test]
fn test_into_num() {
    let index = Index::from(123);
    let num: u32 = index.into();
    assert_eq!(num, 123);
}

impl AsRef<u32> for Index {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

#[test]
fn test_as_ref() {
    let index = Index::from(123);
    assert_eq!(index.as_ref(), &123);
}

impl AsMut<u32> for Index {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

#[test]
fn test_as_mut() {
    let mut index = Index::from(123);
    assert_eq!(index.0, 123);
    {
        let index_ref = index.as_mut();
        *index_ref += 321;
    }
    assert_eq!(index.0, 123 + 321);
}
