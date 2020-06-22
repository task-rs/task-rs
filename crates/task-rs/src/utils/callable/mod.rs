pub mod function;

pub use function::Function;

pub trait Callable {
    type Input;
    type Output;
    fn call(self, x: Self::Input) -> Self::Output;
}

mod references;
