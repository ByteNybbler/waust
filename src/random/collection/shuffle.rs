// The algorithm used here is the Fisher-Yates shuffle.

use crate::random::*;
use std::iter::*;

pub trait ShuffleSlice<T> {
    fn shuffle_first_sample_all<N>(&mut self, rng: &mut N, swap_count: usize)
    where
        N: Rng<usize>;

    fn shuffle_all<N>(&mut self, rng: &mut N)
    where
        N: Rng<usize>;

    fn random_permutation<N>(&mut self, rng: &mut N, length: usize) -> &[T]
    where
        N: Rng<usize>;
}

impl<T> ShuffleSlice<T> for [T] {
    fn shuffle_first_sample_all<N>(&mut self, rng: &mut N, swap_count: usize)
    where
        N: Rng<usize>
    {
        let elements_to_sample = self.len();
        let swap_count = std::cmp::min(swap_count, elements_to_sample);
        for i in 0..swap_count {
            let j = rng.random_range(i, elements_to_sample);
            self.swap(i, j);
        }
    }

    fn shuffle_all<N>(&mut self, rng: &mut N)
    where
        N: Rng<usize>
    {
        self.shuffle_first_sample_all(rng, self.len()-1);
    }

    fn random_permutation<N>(&mut self, rng: &mut N, length: usize) -> &[T]
    where
        N: Rng<usize>
    {
        self.shuffle_first_sample_all(rng, length);
        &self[0..length]
    }
}

pub trait ShuffleIterator<T>
{
    fn shuffle_first_sample_all<N>(self, rng: &mut N, swap_count: usize) -> Vec<T>
    where
        N: Rng<usize>;

    fn shuffle_all<N>(self, rng: &mut N) -> Vec<T>
    where
        N: Rng<usize>;
        
    fn random_permutation<N>(self, rng: &mut N, length: usize) -> Vec<T>
    where
        N: Rng<usize>;
}

impl<I: Iterator> ShuffleIterator<I::Item> for I {
    fn shuffle_first_sample_all<N>(self, rng: &mut N, swap_count: usize) -> Vec<I::Item>
    where
        N: Rng<usize>
    {
        let mut collection: Vec<_> = self.collect();
        collection.shuffle_first_sample_all(rng, swap_count);
        collection
    }

    fn shuffle_all<N>(self, rng: &mut N) -> Vec<I::Item>
    where
        N: Rng<usize>
    {
        let mut collection: Vec<_> = self.collect();
        collection.shuffle_all(rng);
        collection
    }

    fn random_permutation<N>(self, rng: &mut N, length: usize) -> Vec<I::Item>
    where
        N: Rng<usize>
    {
        let mut collection: Vec<_> = self.collect();
        collection.shuffle_first_sample_all(rng, length);
        collection.truncate(length);
        collection
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuffle_slice() {
        let mut rng = Pcg32::from_current_time_and_address();

        let mut array = [1, 2, 3, 4, 5];
        array.shuffle_all(&mut rng);
        array.shuffle_first_sample_all(&mut rng, 800);
        let permutation = array.random_permutation(&mut rng, 3);
        assert_eq!(permutation.len(), 3);

        let slice = &mut [1, 2, 3, 4, 5];
        slice.shuffle_all(&mut rng);
        slice.shuffle_first_sample_all(&mut rng, 800);
        let permutation = slice.random_permutation(&mut rng, 3);
        assert_eq!(permutation.len(), 3);

        let mut vec = vec![1, 2, 3, 4, 5];
        vec.shuffle_all(&mut rng);
        vec.shuffle_first_sample_all(&mut rng, 800);
        let permutation = vec.random_permutation(&mut rng, 3);
        assert_eq!(permutation.len(), 3);
    }

    #[test]
    fn test_shuffle_iterator() {
        let mut rng = Pcg32::from_current_time_and_address();

        let array = [1, 2, 3, 4, 5];
        let iter = array.iter();
        iter.shuffle_all(&mut rng);
        let iter = array.iter();
        iter.shuffle_first_sample_all(&mut rng, 800);
        let iter = array.iter();
        iter.random_permutation(&mut rng, 800);

        let slice = &[1, 2, 3, 4, 5];
        let iter = slice.iter();
        iter.shuffle_all(&mut rng);
        let iter = slice.iter();
        iter.shuffle_first_sample_all(&mut rng, 800);
        let iter = array.iter();
        iter.random_permutation(&mut rng, 800);

        let vec = vec![1, 2, 3, 4, 5];
        let iter = vec.iter();
        iter.shuffle_all(&mut rng);
        let iter = vec.iter();
        iter.shuffle_first_sample_all(&mut rng, 800);
        let iter = array.iter();
        iter.random_permutation(&mut rng, 800);
    }
}