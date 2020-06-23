#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Index(u32);

impl From<u32> for Index {
    fn from(value: u32) -> Self {
        Index(value)
    }
}

impl Into<u32> for Index {
    fn into(self) -> u32 {
        self.0
    }
}

impl AsRef<u32> for Index {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsMut<u32> for Index {
    fn as_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}
