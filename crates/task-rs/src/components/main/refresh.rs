use super::super::controls;
use super::Main;
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

pub trait Refresh<'a> {
    fn refresh(main: &'a mut Main) -> Self;
}

impl<'a> Refresh<'a> for &'a mut Main {
    fn refresh(main: &'a mut Main) -> Self {
        main.controls.tag_list = main
            .data
            .tags
            .iter_index()
            .map(|(index, _)| (index, button::State::default()))
            .collect::<BTreeMap<_, _>>()
            .pipe(controls::TagList);

        main.controls.task_list = main
            .data
            .tasks
            .iter()
            .enumerate()
            .map(|(index, task)| controls::TaskItem::from_task_ref(vec![index], task))
            .collect::<Vec<_>>()
            .pipe(controls::TaskList);

        main
    }
}

impl Main {
    pub fn refresh(&mut self) {
        <&mut Main as Refresh>::refresh(self);
    }
}
