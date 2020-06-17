use super::super::style::Style;

pub trait Theme {
    fn style(&self) -> Style;
}

impl<Type: Theme> Theme for &Type {
    fn style(&self) -> Style {
        (*self).style()
    }
}
