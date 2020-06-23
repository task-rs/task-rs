use super::{Index, IndexedMap, InsertResult, RemoveResult};
use core::fmt::{Debug, Display};
use pipe_trait::*;
use std::{collections::BTreeMap, rc::Rc};

fn assert_eq<Type: PartialEq + Debug>(a: Type, msg: impl Display) -> impl Fn(Type) -> () {
    move |b: Type| assert_eq!(a, b, "{}", msg)
}

fn removed_value<Key, Value>(remove_result: RemoveResult<Key, Value>) -> Option<Value> {
    match remove_result {
        RemoveResult::Removed(_, value) => Some(value),
        RemoveResult::Unchanged => None,
    }
}

const BEFORE: &str = include_str!("./assets/before.yaml");
const AFTER: &str = include_str!("./assets/after.yaml");

type MyStruct = IndexedMap<String, i32>;

impl MyStruct {
    fn with(mut self, key: impl Into<String>, value: i32) -> Self {
        self.insert_key(Rc::new(key.into()), value);
        self
    }

    fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str(yaml).expect("parse IndexedMap from yaml string")
    }

    fn to_yaml(&self) -> String {
        serde_yaml::to_string(self).expect("dump IndexedMap as yaml string")
    }
}

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
}

#[test]
fn test_deserialize_before() {
    MyStruct::default()
        .with("abc", 123)
        .with("jkl", 123)
        .with("def", 456)
        .with("mno", 456)
        .with("ghi", 789)
        .to_yaml()
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
        .to_yaml()
        .pipe(assert_eq(AFTER.trim_end().to_owned(), "yaml comparison"));
}

#[test]
fn test_from_btreemap() {
    let actual = [("abc", 123), ("def", 456), ("ghi", 789)]
        .iter()
        .cloned()
        .map(|(key, value)| (key.to_owned(), value))
        .collect::<BTreeMap<_, _>>()
        .pipe(MyStruct::from);

    let expected = MyStruct::default()
        .with("ghi", 789)
        .with("abc", 123)
        .with("def", 456);

    assert_eq!(actual, expected);
}

#[test]
fn test_into_btreemap() {
    let actual: BTreeMap<_, _> = MyStruct::default()
        .with("ghi", 789)
        .with("abc", 123)
        .with("def", 456)
        .into();

    let expected: BTreeMap<_, _> = [("abc", 123), ("def", 456), ("ghi", 789)]
        .iter()
        .cloned()
        .map(|(key, value)| (key.to_owned(), value))
        .collect();

    assert_eq!(actual, expected);
}

#[test]
fn test_iter() {
    let actual: Vec<_> = MyStruct::default()
        .with("ghi", 789)
        .with("abc", 123)
        .with("def", 456)
        .iter()
        .map(|(key, value)| (key.clone(), *value))
        .collect();

    let expected = vec![
        ("abc".to_owned(), 123),
        ("def".to_owned(), 456),
        ("ghi".to_owned(), 789),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_iter_mut() {
    let mut actual_map = MyStruct::default()
        .with("ghi", 789)
        .with("abc", 123)
        .with("def", 456);

    let mut actual_keys = Vec::new();
    for (key, value) in actual_map.iter_mut() {
        actual_keys.push(key.clone());
        *value = *value * *value;
    }

    let expected_map = MyStruct::default()
        .with("abc", 123 * 123)
        .with("def", 456 * 456)
        .with("ghi", 789 * 789);
    let expected_keys = vec!["abc".to_owned(), "def".to_owned(), "ghi".to_owned()];

    assert_eq!(actual_map, expected_map);
    assert_eq!(actual_keys, expected_keys);
}

#[test]
fn test_iter_index() {
    let actual: Vec<_> = MyStruct::default()
        .with("ghi", 789)
        .with("abc", 123)
        .with("def", 456)
        .iter_index()
        .map(|(key, value)| (key, value.clone()))
        .collect();

    let expected = vec![
        (Index::from(1), "abc".to_owned()),
        (Index::from(2), "def".to_owned()),
        (Index::from(0), "ghi".to_owned()),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn test_insert_remove_key() {
    let mut actual = MyStruct::from_yaml(BEFORE);
    actual
        .remove_key(&Rc::new("mno".to_owned()))
        .pipe(removed_value)
        .pipe(assert_eq(Some(456), "removed value"));
    actual
        .insert_key(Rc::new("foo".to_owned()), 321)
        .pipe(assert_eq(
            5.pipe(Index::from).pipe(InsertResult::Added),
            "added index",
        ));
    actual
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

    let abc = actual.get_index_by_key(&Rc::new("abc".to_owned())).unwrap();
    let ghi = actual.get_index_by_key(&Rc::new("ghi".to_owned())).unwrap();
    actual
        .replace_index(abc, 111)
        .pipe(assert_eq(Some(123), "replaced value of index of 'abc'"));
    actual
        .replace_index(ghi, 333)
        .pipe(assert_eq(Some(789), "replaced value of index of 'ghi'"));
    actual
        .replace_index(Index::from(999), 8)
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

    let abc = actual.get_index_by_key(&Rc::new("abc".to_owned())).unwrap();
    let ghi = actual.get_index_by_key(&Rc::new("ghi".to_owned())).unwrap();
    actual.remove_index(abc).pipe(assert_eq(
        RemoveResult::Removed(Rc::new("abc".to_owned()), 123),
        "replaced value of index of 'abc'",
    ));
    actual.remove_index(ghi).pipe(assert_eq(
        RemoveResult::Removed(Rc::new("ghi".to_owned()), 789),
        "replaced value of index of 'ghi'",
    ));
    actual
        .replace_index(Index::from(999), 8)
        .pipe(assert_eq(None, "replaced value of non-existence index"));

    let expected = MyStruct::default()
        .with("def", 456)
        .with("jkl", 987)
        .with("mno", 654)
        .with("pqr", 321);

    assert_eq!(actual, expected);
}
