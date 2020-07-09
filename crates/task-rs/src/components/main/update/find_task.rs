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
