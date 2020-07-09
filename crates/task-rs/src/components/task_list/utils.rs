use super::super::super::data::Task;
use super::super::task_item::{StatusAccumulation, TaskItem};

pub fn extend_task_item_list(
    target: &mut Vec<TaskItem>,
    tasks: &[Task],
    address_prefix: &[usize],
    task_status_accumulation: StatusAccumulation,
) {
    for (index, task) in tasks.iter().enumerate() {
        let prefix = || [address_prefix, &[index]].concat();
        target.push(TaskItem::from_task_ref(
            prefix(),
            task,
            task_status_accumulation,
        ));
        extend_task_item_list(
            target,
            &task.sub,
            &prefix(),
            task_status_accumulation.join_task(task),
        );
    }
}

#[cfg(test)]
fn load() -> Vec<TaskItem> {
    use pipe_trait::*;
    let tasks: Vec<_> = include_str!("./fixtures/task-items.yaml")
        .pipe(serde_yaml::from_str)
        .unwrap();
    let mut task_items = Vec::new();
    extend_task_item_list(&mut task_items, &tasks, &[], Default::default());

    task_items
}

#[test]
fn test_extend_task_item_list() {
    let task_items = load();

    let actual: Vec<_> = task_items
        .iter()
        .map(|item| (item.task_address.as_slice(), item.task_summary.as_str()))
        .collect();

    let expected: Vec<(&[usize], &str)> = vec![
        (&[0], "first task"),
        (&[1], "task with a sub"),
        (&[1, 0], "first sub task"),
        (&[2], "task with 2 subs"),
        (&[2, 0], "completed sub task"),
        (&[2, 1], "active sub task"),
        (&[3], "deep sub task levels"),
        (&[3, 0], "deep sub task levels 1"),
        (&[3, 0, 0], "deep sub task levels 1a"),
        (&[3, 0, 1], "deep sub task levels 1b"),
        (&[3, 1], "deep sub task levels 2"),
        (&[3, 1, 0], "deep sub task levels 2a"),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn task_status_accumulation() {
    let task_items = load();

    let actual: Vec<_> = task_items
        .iter()
        .map(|item| {
            (
                item.task_address.as_slice(),
                item.task_status_accumulation.all_active,
                item.task_status_accumulation.some_completed,
            )
        })
        .collect();

    let expected: Vec<(&[usize], bool, bool)> = vec![
        (&[0], true, false),
        (&[1], true, false),
        (&[1, 0], true, false),
        (&[2], true, false),
        (&[2, 0], false, true),
        (&[2, 1], true, false),
        (&[3], true, false),
        (&[3, 0], true, false),
        (&[3, 0, 0], false, true),
        (&[3, 0, 1], true, false),
        (&[3, 1], false, true),
        (&[3, 1, 0], false, true),
    ];

    assert_eq!(actual, expected);
}
