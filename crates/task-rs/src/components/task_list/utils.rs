use super::super::super::data::Task;
use super::super::TaskItem;

pub fn extend_task_item_list(target: &mut Vec<TaskItem>, tasks: &[Task], address_prefix: &[usize]) {
    for (index, task) in tasks.iter().enumerate() {
        let prefix = || [address_prefix, &[index]].concat();
        target.push(TaskItem::from_task_ref(prefix(), task));
        extend_task_item_list(target, &task.sub, &prefix());
    }
}

#[test]
fn test_extend_task_item_list() {
    use pipe_trait::*;
    let tasks: Vec<_> = include_str!("./fixtures/task-items.yaml")
        .pipe(serde_yaml::from_str)
        .unwrap();
    let mut task_items = Vec::new();

    extend_task_item_list(&mut task_items, &tasks, &[]);

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
    ];
    assert_eq!(actual, expected);
}