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
