use super::Callable;
use core::marker::PhantomData;

#[derive(Debug, Copy, Clone)]
pub struct Function<F, X, Y> {
    function: F,
    _phantom: PhantomData<(X, Y)>,
}

impl<F, X, Y> Callable for Function<F, X, Y>
where
    F: Fn(X) -> Y,
{
    type Input = X;
    type Output = Y;
    fn call(self, x: Self::Input) -> Self::Output {
        (self.function)(x)
    }
}

impl<F, X, Y> From<F> for Function<F, X, Y>
where
    F: Fn(X) -> Y,
{
    fn from(function: F) -> Self {
        Function {
            function,
            _phantom: Default::default(),
        }
    }
}

#[test]
fn test_function() {
    let f = |x: i32| (x * x) as u64;
    let g = Function::from(&f);
    assert_eq!(g.call(12), f(12));
}
