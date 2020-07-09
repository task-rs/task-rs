use super::super::super::super::data::Task;

pub(crate) fn find_task<'task>(
    target: &'task mut Vec<Task>,
    address: &[usize],
) -> Option<&'task mut Task> {
    if address.is_empty() {
        panic!("address is empty");
    }

    let head = address[0];
    let tail = &address[1..];

    if let Some(task) = target.get_mut(head) {
        if tail.is_empty() {
            return Some(task);
        }

        if task.sub.is_empty() {
            return None;
        }

        return find_task(&mut task.sub, tail);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::super::super::super::super::data::{Data, Task};
    use super::find_task;
    use pipe_trait::*;

    fn load() -> Vec<Task> {
        include_str!("./fixtures/data.yaml")
            .pipe(serde_yaml::from_str::<Data>)
            .expect("parse data")
            .tasks
    }

    #[test]
    #[should_panic(expected = "address is empty")]
    fn empty_address() {
        find_task(&mut load(), &[]);
    }

    #[test]
    fn not_exist() {
        let mut tasks = load();

        macro_rules! test {
            ($address:expr) => {
                let result = find_task(&mut tasks, $address);
                assert_eq!(result, None, "address = {:?}", $address);
            };
        }

        test!(&[0, 0]);
        test!(&[1, 0, 0]);
        test!(&[3, 1, 2, 0]);
        test!(&[123]);
        test!(&[1, 1]);
    }

    #[test]
    fn exist() {
        let mut tasks = load();

        macro_rules! test {
            ($address:expr, $expected:expr) => {
                let result = find_task(&mut tasks, $address);
                assert_eq!(
                    result.cloned(),
                    Some($expected.clone()),
                    "content (address = {:?}, expected = {:?})",
                    $address,
                    stringify!($expected),
                );
                let result = find_task(&mut tasks, $address);
                let actual_address = result.unwrap() as *const _;
                let expected_address = $expected as *const _;
                assert_eq!(
                    actual_address,
                    expected_address,
                    "memory address (address = {:?}, expected = {:?})",
                    $address,
                    stringify!($expected),
                );
            };
        }

        test!(&[0], &tasks[0]);
        test!(&[1], &tasks[1]);
        test!(&[1, 0], &tasks[1].sub[0]);
        test!(&[2], &tasks[2]);
        test!(&[2, 0], &tasks[2].sub[0]);
        test!(&[3], &tasks[3]);
        test!(&[3, 0], &tasks[3].sub[0]);
        test!(&[3, 0, 0], &tasks[3].sub[0].sub[0]);
        test!(&[3, 0, 1], &tasks[3].sub[0].sub[1]);
        test!(&[3, 1], &tasks[3].sub[1]);
        test!(&[3, 1, 0], &tasks[3].sub[1].sub[0]);
    }
}
