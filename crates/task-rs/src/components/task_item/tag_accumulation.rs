use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Clone, Copy, Eq, PartialEq)]
pub struct TagAccumulation {
    #[default(false)]
    pub satisfaction: bool,
}

impl TagAccumulation {
    pub fn from_bool(satisfaction: bool) -> Self {
        TagAccumulation { satisfaction }
    }
}
