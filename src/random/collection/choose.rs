use crate::random::*;

pub trait ChooseSlice<T> {
    fn choose<N>(&self, rng: &mut N) -> &T
    where
        N: Rng<usize>;

    fn choosen<N>(&self, rng: &mut N, count: usize) -> Vec<&T>
    where
        N: Rng<usize>;
}

impl<T> ChooseSlice<T> for [T] {
    fn choose<N>(&self, rng: &mut N) -> &T
    where
        N: Rng<usize>
    {
        &self[rng.random_bounded(self.len())]
    }

    fn choosen<N>(&self, rng: &mut N, count: usize) -> Vec<&T>
    where
        N: Rng<usize>
    {
        let mut result = vec![];
        for _ in 0..count {
            result.push(self.choose(rng));
        }
        result
    }
}