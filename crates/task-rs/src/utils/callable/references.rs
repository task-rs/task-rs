use super::Callable;

impl<F, X, Y> Callable for &F
where
    F: Callable<Input = X, Output = Y> + Copy,
{
    type Input = X;
    type Output = Y;
    fn call(self, x: X) -> Y {
        (*self).call(x)
    }
}

#[test]
fn test_reference() {
    use super::Function;
    let f = |x: i32| x.to_string();
    let g = Function::from(f);
    let g = &g;
    assert_eq!(g.call(12), f(12));
}
