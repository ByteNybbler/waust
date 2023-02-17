/*
 * PCG Random Number Generation was adapted for the Rust programming language.
 * The original notice was as follows.
 */

/*
 * PCG Random Number Generation for C.
 *
 * Copyright 2014 Melissa O'Neill <oneill@pcg-random.org>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 * For additional information about the PCG random number generation scheme,
 * including its license and other licensing options, visit
 *
 *       http://www.pcg-random.org
 */

use crate::random::*;
use crate::time::*;

// For generating 64-bit values without requiring 128-bit integers.
pub struct Pcg32x2 {
    gen1: Pcg32,
    gen2: Pcg32
}

impl Pcg32x2 {
    pub fn new(seed1: u64, seed2: u64, seq1: u64, mut seq2: u64) -> Pcg32x2 {
        let mask = !0 >> 1;
        // The stream for each of the two generators must be distinct.
        if seq1 & mask == seq2 & mask {
            seq2 = !seq2;
        }
        Pcg32x2 {
            gen1: Pcg32::new(seed1, seq1),
            gen2: Pcg32::new(seed2, seq2)
        }
    }

    pub fn from_current_time_and_address() -> Pcg32x2 {
        let time = get_current_unix_timestamp();
        let address1 = &time as *const u64 as u64;
        let address2 = &address1 as *const _ as u64;
        let address3 = &address2 as *const _ as u64;
        let address4 = &address3 as *const _ as u64;
        //println!("{} {} {} {}", address1, address2, address3, address4);
        Self::new(time ^ address1, !time ^ address2, address3, address4)
    }
}

impl Rng<u64> for Pcg32x2 {
    fn random(&mut self) -> u64 {
        let first: u32 = self.gen1.random();
        let second: u32 = self.gen2.random();
        let result_part = (first as u64) << 32;
        result_part | second as u64
    }

    fn random_bounded(&mut self, max_exclusive: u64) -> u64 {
        let threshold = (u64::MAX - max_exclusive + 1) % max_exclusive;
        loop {
            let r: u64 = self.random();
            if r >= threshold {
                return r % max_exclusive
            }
        }
    }

    fn random_range(&mut self, min_inclusive: u64, max_exclusive: u64) -> u64 {
        min_inclusive + self.random_bounded(max_exclusive.wrapping_sub(min_inclusive))
    }
}

impl Rng<i64> for Pcg32x2 {
    fn random(&mut self) -> i64 {
        Rng::<u64>::random(self) as i64
    }

    fn random_bounded(&mut self, max_exclusive: i64) -> i64 {
        Rng::<u64>::random_bounded(self, max_exclusive as u64) as i64
    }

    fn random_range(&mut self, min_inclusive: i64, max_exclusive: i64) -> i64 {
        Rng::<u64>::random_range(self, min_inclusive as u64, max_exclusive as u64) as i64
    }
}

impl Rng<f64> for Pcg32x2 {
    fn random(&mut self) -> f64 {
        // This distribution is not uniform! Very very small outputs are underrepresented.
        Rng::<u64>::random(self) as f64 * 2f64.powi(-64)
    }

    fn random_bounded(&mut self, max_exclusive: f64) -> f64 {
        Rng::<f64>::random(self) * max_exclusive
    }

    fn random_range(&mut self, min_inclusive: f64, max_exclusive: f64) -> f64 {
        min_inclusive + Rng::<f64>::random_bounded(self, max_exclusive - min_inclusive)
    }
}

impl Rng<u32> for Pcg32x2 {
    fn random(&mut self) -> u32 {
        self.gen1.random()
    }

    fn random_bounded(&mut self, max_exclusive: u32) -> u32 {
        self.gen1.random_bounded(max_exclusive)
    }

    fn random_range(&mut self, min_inclusive: u32, max_exclusive: u32) -> u32 {
        self.gen1.random_range(min_inclusive, max_exclusive)
    }
}

impl Rng<i32> for Pcg32x2 {
    fn random(&mut self) -> i32 {
        self.gen1.random()
    }

    fn random_bounded(&mut self, max_exclusive: i32) -> i32 {
        self.gen1.random_bounded(max_exclusive)
    }

    fn random_range(&mut self, min_inclusive: i32, max_exclusive: i32) -> i32 {
        self.gen1.random_range(min_inclusive, max_exclusive)
    }
}

impl Rng<f32> for Pcg32x2 {
    fn random(&mut self) -> f32 {
        self.gen1.random()
    }

    fn random_bounded(&mut self, max_exclusive: f32) -> f32 {
        self.gen1.random_bounded(max_exclusive)
    }

    fn random_range(&mut self, min_inclusive: f32, max_exclusive: f32) -> f32 {
        self.gen1.random_range(min_inclusive, max_exclusive)
    }
}

impl Rng<usize> for Pcg32x2 {
    fn random(&mut self) -> usize {
        Rng::<u64>::random(self) as usize
    }

    fn random_bounded(&mut self, max_exclusive: usize) -> usize {
        Rng::<u64>::random_bounded(self, max_exclusive as u64) as usize
    }

    fn random_range(&mut self, min_inclusive: usize, max_exclusive: usize) -> usize {
        Rng::<u64>::random_range(self, min_inclusive as u64, max_exclusive as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcg32x2() {
        let mut rng = Pcg32x2::from_current_time_and_address();
        let r: u64 = rng.random_range(10, 15);
        assert!(10 <= r && r < 15);
        let r: i64 = rng.random_range(-15, -10);
        assert!(-15 <= r && r < -10);
        let r: f64 = rng.random_range(10.0, 15.0);
        assert!(10.0 <= r && r < 15.0);
        let r: u32 = rng.random_range(10, 15);
        assert!(10 <= r && r < 15);
        let r: i32 = rng.random_range(-15, -10);
        assert!(-15 <= r && r < -10);
        let r: f32 = rng.random_range(10.0, 15.0);
        assert!(10.0 <= r && r < 15.0);
    }
}