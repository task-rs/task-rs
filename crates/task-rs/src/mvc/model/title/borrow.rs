use super::Title;
use core::borrow::{Borrow, BorrowMut};

impl Borrow<str> for Title {
    fn borrow(&self) -> &str {
        Borrow::<String>::borrow(self)
    }
}

impl Borrow<String> for Title {
    fn borrow(&self) -> &String {
        &self.0
    }
}

impl BorrowMut<String> for Title {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

#[test]
fn test_borrow() {
    let title = Title::from("foo");
    let content: &str = title.borrow();
    assert_eq!(content, "foo");
}

#[test]
fn test_borrow_mut() {
    let mut title = Title::from("foo");
    assert_eq!(title.borrow() as &str, "foo");
    {
        let content: &mut String = title.borrow_mut();
        *content = "bar".to_owned();
    }
    assert_eq!(title.borrow() as &str, "bar");
}
