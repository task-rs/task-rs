use super::super::super::data::Task;
use super::super::TaskItem;

pub fn extend_task_item_list(target: &mut Vec<TaskItem>, tasks: &[Task], address_prefix: &[usize]) {
    for (index, task) in tasks.iter().enumerate() {
        let prefix = || [address_prefix, &[index]].concat();
        target.push(TaskItem::from_task_ref(prefix(), task));
        extend_task_item_list(target, &task.sub, &prefix());
    }
}
