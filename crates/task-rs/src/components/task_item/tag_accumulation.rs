use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Clone, Copy, Eq, PartialEq)]
pub struct TagAccumulation {
    #[default(false)]
    pub satisfaction: bool,
}

impl TagAccumulation {
    pub fn join_satisfaction_func(self, func: impl FnOnce() -> bool) -> Self {
        TagAccumulation {
            satisfaction: self.satisfaction || func(),
        }
    }
}
