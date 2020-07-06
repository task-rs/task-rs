use super::Main;

pub trait Refresh<'a> {
    fn refresh(main: &'a mut Main) -> Self;
}

impl<'a> Refresh<'a> for &'a mut Main {
    fn refresh(main: &'a mut Main) -> Self {
        main.controls.tag_list = Refresh::refresh(main);
        main.controls.task_list = Refresh::refresh(main);
        main
    }
}

impl Main {
    pub fn refresh(&mut self) {
        <&mut Main as Refresh>::refresh(self);
    }
}
