use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Copy, Clone, Eq, PartialEq)]
pub struct StatusAccumulation {
    #[default(true)]
    all_active: bool,
    #[default(false)]
    some_completed: bool,
}

impl StatusAccumulation {
    pub fn join(self, other: Self) -> Self {
        self.join_all_active(other.all_active)
            .join_some_completed(other.some_completed)
    }

    pub fn join_all_active(self, other: bool) -> Self {
        StatusAccumulation {
            all_active: self.all_active && other,
            ..self
        }
    }

    pub fn join_some_completed(self, other: bool) -> Self {
        StatusAccumulation {
            some_completed: self.some_completed || other,
            ..self
        }
    }
}
