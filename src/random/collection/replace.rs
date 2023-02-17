use crate::random::*;

pub trait ReplaceSlice<T>
{
    fn replace_target<E, F>(&mut self, target: E, replacement: F)
    where
        E: ReplacementPattern<T>,
        F: FnMut() -> T;
    
    fn swap_targets(&mut self, first: T, second: T)
    where
        T: PartialEq + Copy;

    fn randomly_replace_any_count<N, F>(&mut self, rng: &mut N, replacement: F, count: usize)
    where
        N: Rng<usize>,
        F: FnMut(&mut N) -> T;

    fn randomly_replace_any_chance<N, F>(&mut self, rng: &mut N, replacement: F, chance: Chance)
    where
        N: Rng<u32>,
        F: FnMut(&mut N) -> T;

    fn randomly_replace_target_count<N, E, F>(&mut self, rng: &mut N, target: E, replacement: F, count: usize)
    where
        N: Rng<usize>,
        E: ReplacementPattern<T>,
        F: FnMut(&mut N) -> T;

    fn randomly_replace_target_chance<N, E, F>(&mut self, rng: &mut N, target: E, replacement: F, chance: Chance)
    where
        N: Rng<u32>,
        E: ReplacementPattern<T>,
        F: FnMut(&mut N) -> T;
}

impl<T> ReplaceSlice<T> for [T]
{
    fn replace_target<E, F>(&mut self, target: E, mut replacement: F)
    where
        E: ReplacementPattern<T>,
        F: FnMut() -> T
    {
        for t in self {
            if target.satisfied_by(t) {
                *t = replacement();
            }
        }
    }

    fn swap_targets(&mut self, first: T, second: T)
    where
        T: PartialEq + Copy
    {
        for t in self {
            if *t == first {
                *t = second;
            } else if *t == second {
                *t = first;
            }
        }
    }

    fn randomly_replace_any_count<N, F>(&mut self, rng: &mut N, mut replacement: F, count: usize)
    where
        N: Rng<usize>,
        F: FnMut(&mut N) -> T
    {
        let targets = self.iter_mut().random_permutation(rng, count);
        for t in targets {
            *t = replacement(rng);
        }
    }

    fn randomly_replace_any_chance<N, F>(&mut self, rng: &mut N, mut replacement: F, chance: Chance)
    where
        N: Rng<u32>,
        F: FnMut(&mut N) -> T
    {
        for t in self {
            if chance.happens(rng) {
                *t = replacement(rng);
            }
        }
    }

    fn randomly_replace_target_count<N, E, F>(&mut self, rng: &mut N, target: E, mut replacement: F, count: usize)
    where
        N: Rng<usize>,
        E: ReplacementPattern<T>,
        F: FnMut(&mut N) -> T
    {
        let targets = self.iter_mut().filter(|t| target.satisfied_by(t)).random_permutation(rng, count);
        for t in targets {
            *t = replacement(rng);
        }
    }

    fn randomly_replace_target_chance<N, E, F>(&mut self, rng: &mut N, target: E, mut replacement: F, chance: Chance)
    where
        N: Rng<u32>,
        E: ReplacementPattern<T>,
        F: FnMut(&mut N) -> T
    {
        for t in self.iter_mut().filter(|t| target.satisfied_by(t)) {
            if chance.happens(rng) {
                *t = replacement(rng);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_deterministic() {
        let mut array = [1, 2, 3, 4, 5];
        array.replace_target(3, || 5);
        assert_eq!(array, [1, 2, 5, 4, 5]);
        array.replace_target(|num: &i32| *num > 3, || 0);
        assert_eq!(array, [1, 2, 0, 0, 0]);
    }
}