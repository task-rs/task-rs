use super::super::super::data::{Status, TagId, Task};
use super::super::task_item::{StatusAccumulation, TaskItem};
use std::collections::BTreeSet;

fn accumulate_tags(mut target: BTreeSet<TagId>, addend: &BTreeSet<TagId>) -> BTreeSet<TagId> {
    for item in addend {
        if !target.contains(item) {
            target.insert(item.clone());
        }
    }

    target
}

fn extend_task_item_list(
    target: &mut Vec<TaskItem>,
    tasks: &[Task],
    address_prefix: &[usize],
    status_accumulation: StatusAccumulation,
    tag_accumulation: BTreeSet<TagId>,
) {
    for (index, task) in tasks.iter().enumerate() {
        let prefix = || [address_prefix, &[index]].concat();
        let tag_accumulation = accumulate_tags(tag_accumulation.clone(), &task.tags);
        let mut item = TaskItem::from_task_ref(prefix(), task, status_accumulation);
        item.tag_accumulation.tags = tag_accumulation.clone();
        target.push(item);
        extend_task_item_list(
            target,
            &task.sub,
            &prefix(),
            status_accumulation.join_task(task),
            tag_accumulation,
        );
    }
}

fn calculate_contains_completed(target: &mut [TaskItem]) {
    let len = target.len();
    for i in 0..len {
        // `i..len` instead of `(i + 1)..len` so that it skips when `target[i]` "contains completed"
        for j in i..len {
            // if `i`'s address is not prefix of `j`'s address, end loop
            if !target[j].address.starts_with(target[i].address.as_slice()) {
                break;
            }

            // if `j` is completed, mark `i` as "contains completed" and end loop
            if target[j].status == Status::Completed {
                target[i].status_accumulation.contains_completed = true;
                break;
            }
        }
    }
}

fn accumulate_tags_bottom_up(target: &mut [TaskItem]) {
    let len = target.len();
    for i in 0..len {
        for j in i..len {
            if !target[j].address.starts_with(target[i].address.as_slice()) {
                break;
            }

            target[i].tag_accumulation.tags = accumulate_tags(
                target[i].tag_accumulation.tags.clone(),
                &target[j].tag_accumulation.tags,
            );
        }
    }
}

fn calculate_tag_satisfaction(target: &mut [TaskItem], tags: &BTreeSet<TagId>) {
    for item in target.iter_mut() {
        item.tag_accumulation.satisfy = !tags.is_disjoint(&item.tag_accumulation.tags);
    }
}

pub fn create_task_item_list(tasks: &[Task], tags: &Option<BTreeSet<TagId>>) -> Vec<TaskItem> {
    let mut result = Vec::new();
    extend_task_item_list(
        &mut result,
        tasks,
        &[],
        Default::default(),
        Default::default(),
    );
    calculate_contains_completed(&mut result);
    accumulate_tags_bottom_up(&mut result);
    if let Some(tags) = tags {
        calculate_tag_satisfaction(&mut result, tags);
    }
    result
}

#[cfg(test)]
fn load(tags: &Option<BTreeSet<TagId>>) -> Vec<TaskItem> {
    use pipe_trait::*;
    include_str!("./fixtures/task-items.yaml")
        .pipe(serde_yaml::from_str::<Vec<Task>>)
        .unwrap()
        .pipe_ref(|x| create_task_item_list(x, tags))
}

#[test]
fn test_extend_task_item_list() {
    let task_items = load(&None);

    let actual: Vec<_> = task_items
        .iter()
        .map(|item| (item.address.as_slice(), item.summary.as_str()))
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
    let task_items = load(&None);

    let actual: Vec<_> = task_items
        .iter()
        .map(|item| {
            (
                item.address.as_slice(),
                item.status_accumulation.all_active,
                item.status_accumulation.some_completed,
                item.status_accumulation.contains_completed,
            )
        })
        .collect();

    let expected: Vec<(&[usize], bool, bool, bool)> = vec![
        (&[0], true, false, false),
        (&[1], true, false, false),
        (&[1, 0], true, false, false),
        (&[2], true, false, true),
        (&[2, 0], false, true, true),
        (&[2, 1], true, false, false),
        (&[3], true, false, true),
        (&[3, 0], true, false, true),
        (&[3, 0, 0], false, true, true),
        (&[3, 0, 1], true, false, false),
        (&[3, 1], false, true, true),
        (&[3, 1, 0], false, true, false),
    ];

    assert_eq!(actual, expected);
}
