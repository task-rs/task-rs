use super::super::controls;
use super::Main;
use iced::*;
use pipe_trait::*;
use std::collections::BTreeMap;

impl Main {
    pub fn refresh(&mut self) {
        self.controls.tag_list = self
            .data
            .tags
            .iter_index()
            .map(|(index, _)| (index, button::State::default()))
            .collect::<BTreeMap<_, _>>()
            .pipe(controls::TagList);

        self.controls.task_list = self
            .data
            .tasks
            .iter()
            .enumerate()
            .map(|(index, task)| controls::TaskItem::from_task_ref(vec![index], task))
            .collect::<Vec<_>>()
            .pipe(controls::TaskList);
    }
}
