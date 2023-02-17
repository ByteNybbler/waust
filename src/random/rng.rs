mod pcg32;
pub use pcg32::*;
mod pcg32x2;
pub use pcg32x2::*;

pub trait Rng<T> {
    fn random(&mut self) -> T;
    fn random_bounded(&mut self, max_exclusive: T) -> T;
    fn random_range(&mut self, min_inclusive: T, max_exclusive: T) -> T;
}