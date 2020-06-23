use super::{
    methods::{InsertResult, RemoveResult},
    IndexedMap,
};
use core::fmt::{Debug, Display};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

fn assert_eq<Type: PartialEq + Debug>(a: Type, msg: impl Display) -> impl Fn(Type) -> () {
    move |b: Type| assert_eq!(a, b, "{}", msg)
}

fn removed_value<Key, Value>(remove_result: RemoveResult<Key, Value>) -> Option<Value> {
    match remove_result {
        RemoveResult::Removed(_, value) => Some(value),
        RemoveResult::Unchanged => None,
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Clone)]
#[serde(rename_all = "kebab-case")]
struct MyStruct {
    indexed_map: IndexedMap<String, i32>,
}

impl MyStruct {
    fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str(yaml).unwrap()
    }

    fn with<Key: Into<String>>(mut self, key: Key, value: i32) -> Self {
        self.indexed_map.insert_key(Rc::new(key.into()), value);
        self
    }
}

const BEFORE: &str = include_str!("./assets/before.yaml");
const AFTER: &str = include_str!("./assets/after.yaml");

#[test]
fn test_serialize_before() {
    let actual = MyStruct::from_yaml(BEFORE);
    let expected = MyStruct::default()
        .with("abc", 123)
        .with("jkl", 123)
        .with("def", 456)
        .with("mno", 456)
        .with("ghi", 789);
    assert_eq!(&actual, &expected);
    assert_ne!(
        &actual.indexed_map.index_key,
        &expected.indexed_map.index_key
    );
    assert_ne!(
        &actual.indexed_map.key_index,
        &expected.indexed_map.key_index
    );
}

#[test]
fn test_serialize_after() {
    let actual = MyStruct::from_yaml(AFTER);
    let expected = MyStruct::default()
        .with("foo", 321)
        .with("abc", 123)
        .with("def", 456)
        .with("ghi", 789)
        .with("jkl", 654);
    assert_eq!(&actual, &expected);
    assert_ne!(
        &actual.indexed_map.index_key,
        &expected.indexed_map.index_key
    );
    assert_ne!(
        &actual.indexed_map.key_index,
        &expected.indexed_map.key_index
    );
}

#[test]
fn test_deserialize_before() {
    MyStruct::default()
        .with("abc", 123)
        .with("jkl", 123)
        .with("def", 456)
        .with("mno", 456)
        .with("ghi", 789)
        .pipe_ref(serde_yaml::to_string)
        .unwrap()
        .pipe(assert_eq(BEFORE.trim_end().to_owned(), "yaml comparison"));
}

#[test]
fn test_deserialize_after() {
    MyStruct::default()
        .with("foo", 321)
        .with("abc", 123)
        .with("def", 456)
        .with("ghi", 789)
        .with("jkl", 654)
        .pipe_ref(serde_yaml::to_string)
        .unwrap()
        .pipe(assert_eq(AFTER.trim_end().to_owned(), "yaml comparison"));
}

#[test]
fn test_insert_remove_key() {
    let mut actual = MyStruct::from_yaml(BEFORE);
    actual
        .indexed_map
        .remove_key(&Rc::new("mno".to_owned()))
        .pipe(removed_value)
        .pipe(assert_eq(Some(456), "removed value"));
    actual
        .indexed_map
        .insert_key(Rc::new("foo".to_owned()), 321)
        .pipe(assert_eq(InsertResult::Added(5), "added index"));
    actual
        .indexed_map
        .insert_key(Rc::new("jkl".to_owned()), 654)
        .pipe(assert_eq(InsertResult::Replaced(123), "replaced value"));
    let expected = MyStruct::from_yaml(AFTER);
    assert_eq!(actual, expected);
}

#[test]
fn test_replace_index() {
    let mut actual = MyStruct::default()
        .with("abc", 123)
        .with("def", 456)
        .with("ghi", 789)
        .with("jkl", 987)
        .with("mno", 654)
        .with("pqr", 321);

    let abc = actual
        .indexed_map
        .get_index_by_key(&Rc::new("abc".to_owned()))
        .unwrap();
    let ghi = actual
        .indexed_map
        .get_index_by_key(&Rc::new("ghi".to_owned()))
        .unwrap();
    actual
        .indexed_map
        .replace_index(abc, 111)
        .pipe(assert_eq(Some(123), "replaced value of index of 'abc'"));
    actual
        .indexed_map
        .replace_index(ghi, 333)
        .pipe(assert_eq(Some(789), "replaced value of index of 'ghi'"));
    actual
        .indexed_map
        .replace_index(999, 8)
        .pipe(assert_eq(None, "replaced value of non-existence index"));

    let expected = MyStruct::default()
        .with("abc", 111)
        .with("def", 456)
        .with("ghi", 333)
        .with("jkl", 987)
        .with("mno", 654)
        .with("pqr", 321);

    assert_eq!(actual, expected);
}

#[test]
fn test_remove_index() {
    let mut actual = MyStruct::default()
        .with("abc", 123)
        .with("def", 456)
        .with("ghi", 789)
        .with("jkl", 987)
        .with("mno", 654)
        .with("pqr", 321);

    let abc = actual
        .indexed_map
        .get_index_by_key(&Rc::new("abc".to_owned()))
        .unwrap();
    let ghi = actual
        .indexed_map
        .get_index_by_key(&Rc::new("ghi".to_owned()))
        .unwrap();
    actual.indexed_map.remove_index(abc).pipe(assert_eq(
        RemoveResult::Removed(Rc::new("abc".to_owned()), 123),
        "replaced value of index of 'abc'",
    ));
    actual.indexed_map.remove_index(ghi).pipe(assert_eq(
        RemoveResult::Removed(Rc::new("ghi".to_owned()), 789),
        "replaced value of index of 'ghi'",
    ));
    actual
        .indexed_map
        .replace_index(999, 8)
        .pipe(assert_eq(None, "replaced value of non-existence index"));

    let expected = MyStruct::default()
        .with("def", 456)
        .with("jkl", 987)
        .with("mno", 654)
        .with("pqr", 321);

    assert_eq!(actual, expected);
}
