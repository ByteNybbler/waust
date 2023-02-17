pub trait ReplacementPattern<T> {
    fn satisfied_by(&self, other: &T) -> bool;
}

impl<T, F> ReplacementPattern<T> for F
where
    F: Fn(&T) -> bool
{
    fn satisfied_by(&self, other: &T) -> bool {
        self(other)
    }
}

impl ReplacementPattern<i32> for i32
{
    fn satisfied_by(&self, other: &i32) -> bool {
        self == other
    }
}

impl ReplacementPattern<u32> for u32
{
    fn satisfied_by(&self, other: &u32) -> bool {
        self == other
    }
}

// TODO: Implement for other primitive types.