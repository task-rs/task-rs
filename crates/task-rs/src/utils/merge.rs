use std::collections::BTreeSet;

pub fn merge_sets_with_minimal_cloning<Item: Ord + Clone>(
    target: &mut BTreeSet<Item>,
    source: &BTreeSet<Item>,
) {
    for item in source {
        if !target.contains(item) {
            target.insert(item.clone());
        }
    }
}
