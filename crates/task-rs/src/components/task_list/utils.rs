use super::super::super::data::{Status, TagId, Task};
use super::super::task_item::{StatusAccumulation, TagAccumulation, TaskItem};
use std::collections::BTreeSet;

fn extend_task_item_list(
    target: &mut Vec<TaskItem>,
    tasks: &[Task],
    tags: &Option<BTreeSet<TagId>>, // None when filter does not apply (i.e. "Show All")
    address_prefix: &[usize],
    status_accumulation: StatusAccumulation,
    tag_accumulation: TagAccumulation,
) {
    for (index, task) in tasks.iter().enumerate() {
        let prefix = || [address_prefix, &[index]].concat();
        let mut item = TaskItem::from_task_ref(prefix(), task, status_accumulation);
        item.tag_accumulation = tag_accumulation;
        target.push(item);
        extend_task_item_list(
            target,
            &task.sub,
            tags,
            &prefix(),
            status_accumulation.join_task(task),
            if let Some(tags) = tags {
                tag_accumulation.join_satisfaction_func(|| {
                    tag_accumulation.satisfaction || !task.tags.is_disjoint(tags)
                })
            } else {
                Default::default() // <- this value can be whatever, it is irrelevant
            },
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

fn calculate_tag_satisfaction_bottom_up(target: &mut [TaskItem]) {
    let len = target.len();
    for i in 0..len {
        // `i..len` instead of `(i + 1)..len` so that it skips when `target[i]` satisfies
        for j in i..len {
            // if `i`'s address is not prefix of `j`'s address, end loop
            if !target[j].address.starts_with(target[i].address.as_slice()) {
                break;
            }

            // if `j` satisfies, mark `i` as "satisfied" and end loop
            if target[j].tag_accumulation.satisfaction {
                target[i].tag_accumulation.satisfaction = true;
                break;
            }
        }
    }
}

pub fn create_task_item_list(tasks: &[Task], tags: &Option<BTreeSet<TagId>>) -> Vec<TaskItem> {
    let mut result = Vec::new();
    extend_task_item_list(
        &mut result,
        tasks,
        tags,
        &[],
        Default::default(),
        Default::default(),
    );
    calculate_contains_completed(&mut result);
    if tags.is_some() {
        calculate_tag_satisfaction_bottom_up(&mut result);
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

#[cfg(test)]
fn load_with_tags(tags: &[&str]) -> Vec<TaskItem> {
    use pipe_trait::*;
    tags.iter()
        .map(TagId::from)
        .collect::<BTreeSet<_>>()
        .pipe(Some)
        .pipe_ref(load)
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

#[test]
fn tag_accumulation_no_filter() {
    for item in load(&None) {
        assert_eq!(
            item.tag_accumulation.satisfaction, false,
            "address = {:?}",
            item.address
        );
    }
}

#[test]
fn tag_accumulation_filter_no_tags() {
    let task_items = load_with_tags(&[] as &[&str]);

    let actual: Vec<_> = task_items
        .iter()
        .map(|item| (item.address.as_slice(), item.tag_accumulation.satisfaction))
        .collect();

    let expected: Vec<(&[usize], bool)> = vec![
        (&[0], false),
        (&[1], false),
        (&[1, 0], false),
        (&[2], false),
        (&[2, 0], false),
        (&[2, 1], false),
        (&[3], false),
        (&[3, 0], false),
        (&[3, 0, 0], false),
        (&[3, 0, 1], false),
        (&[3, 1], false),
        (&[3, 1, 0], false),
    ];

    assert_eq!(actual, expected);
}
