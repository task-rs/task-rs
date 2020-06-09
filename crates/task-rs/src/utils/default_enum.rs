#[macro_export]
macro_rules! default_enum {
    ($id:ident::$variant:ident) => {
        impl Default for $id {
            fn default() -> Self {
                $id::$variant
            }
        }
    };
}

#[test]
fn test_default_enum() {
    #[derive(Debug, Eq, PartialEq)]
    #[allow(dead_code)]
    enum Foo {
        A,
        B,
        C,
        D,
    }

    default_enum!(Foo::C);

    assert_eq!(Foo::default(), Foo::C);
}
