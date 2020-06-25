use super::{BidirectionalMap, Overwritten};
use core::iter::FromIterator;
use std::rc::Rc;

macro_rules! assert_eq_map {
    ($a:expr, $b:expr) => {
        assert_eq!($a, $b, "two maps are equal");
        assert_eq!($a.lr, $b.lr, "two lr maps are equal");
        assert_eq!($a.rl, $b.rl, "two rl maps are equal");
    };
}

macro_rules! assert_ne_map {
    ($a:expr, $b:expr) => {
        assert_ne!($a, $b, "two maps are not equal");
        assert_ne!($a.lr, $b.lr, "two lr maps are not equal");
        assert_ne!($a.rl, $b.rl, "two rl maps are not equal");
    };
}

#[test]
fn default() {
    let empty_map = BidirectionalMap::<i32, i32>::default();
    assert!(empty_map.lr.is_empty(), "lr is empty");
    assert!(empty_map.rl.is_empty(), "rl is empty");
}

#[test]
fn eq() {
    let a = BidirectionalMap::from_iter(vec![("abc", 789), ("def", 456), ("ghi", 123)]);
    let b = BidirectionalMap::from_iter(vec![("ghi", 123), ("def", 456), ("abc", 789)]);
    assert_eq_map!(a, b);
}

#[test]
fn ne() {
    let a = BidirectionalMap::from_iter(vec![("abc", 123), ("def", 456), ("ghi", 789)]);
    let b = BidirectionalMap::from_iter(vec![("ghi", 123), ("def", 456), ("abc", 789)]);
    assert_ne_map!(a, b);
}

#[test]
fn inverse() {
    let actual =
        BidirectionalMap::from_iter(vec![("abc", 123), ("def", 456), ("ghi", 789)]).inverse();
    let expected = BidirectionalMap::from_iter(vec![(123, "abc"), (456, "def"), (789, "ghi")]);
    assert_eq_map!(actual, expected);
}

#[test]
fn map_ref() {
    let map = BidirectionalMap::from_iter(vec![("abc", 123), ("def", 456), ("ghi", 789)]);
    assert_eq!(map.left_to_right_map(), &map.lr, "left to right");
    assert_eq!(map.right_to_left_map(), &map.rl, "left to right");
}

#[test]
fn insert() {
    let mut map = BidirectionalMap::default();

    for (left, right) in &[
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
    ] {
        let insert_result = map.insert(Rc::new(*left), Rc::new(*right));
        assert_eq!(
            insert_result,
            Overwritten::Neither,
            "insert ({:?}, {:?}) into map",
            left,
            right,
        );
    }

    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('c', 2),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    let insert_result = map.insert(Rc::new('c'), Rc::new(33));
    assert_eq!(
        insert_result,
        Overwritten::RightOnly(Rc::new(2)),
        "insert ('c', 33) into map"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('c', 33),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    let insert_result = map.insert(Rc::new('z'), Rc::new(6));
    assert_eq!(
        insert_result,
        Overwritten::LeftOnly(Rc::new('g')),
        "insert ('z', 6) into map"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('c', 33),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('z', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    let insert_result = map.insert(Rc::new('a'), Rc::new(3));
    assert_eq!(
        insert_result,
        Overwritten::Both(Rc::new('d'), Rc::new(0)),
        "insert ('a', 3) into map"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 3),
            ('b', 1),
            ('c', 33),
            ('e', 4),
            ('f', 5),
            ('z', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    let insert_result = map.insert(Rc::new('o'), Rc::new(123));
    assert_eq!(
        insert_result,
        Overwritten::Neither,
        "insert ('o', 123) into map"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 3),
            ('b', 1),
            ('c', 33),
            ('e', 4),
            ('f', 5),
            ('z', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
            ('o', 123),
        ])
    );
}

#[test]
fn remove() {
    let mut map = BidirectionalMap::from_iter(vec![
        ('a', 0),
        ('b', 1),
        ('c', 2),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
    ]);

    assert_eq!(
        map.remove_by_left(&'c'),
        Some(Rc::new(2)),
        "removed by left: 'c'"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    assert_eq!(
        map.remove_by_left(&'c'),
        None,
        "removed by left: 'c' (again)"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('h', 7),
            ('i', 8),
            ('j', 9),
        ])
    );

    assert_eq!(
        map.remove_by_right(&7),
        Some(Rc::new('h')),
        "removed by right: 7"
    );
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('i', 8),
            ('j', 9),
        ])
    );

    assert_eq!(map.remove_by_right(&7), None, "removed by right: 7 (again)");
    assert_eq_map!(
        map,
        BidirectionalMap::from_iter(vec![
            ('a', 0),
            ('b', 1),
            ('d', 3),
            ('e', 4),
            ('f', 5),
            ('g', 6),
            ('i', 8),
            ('j', 9),
        ])
    );
}

#[test]
fn clear() {
    let mut map = BidirectionalMap::from_iter(vec![
        ('a', 0),
        ('b', 1),
        ('d', 3),
        ('e', 4),
        ('f', 5),
        ('g', 6),
        ('h', 7),
        ('i', 8),
        ('j', 9),
    ]);

    map.clear();
    assert!(map.lr.is_empty(), "lr is empty");
    assert!(map.rl.is_empty(), "rl is empty");
}
