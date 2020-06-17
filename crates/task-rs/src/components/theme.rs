use super::super::style::Style;

pub trait Theme {
    fn style(&self) -> Style;
}
