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

// For generating 32-bit values.
pub struct Pcg32 {
    state: u64,
    inc: u64
}

impl Pcg32 {
    pub fn new(seed: u64, seq: u64) -> Pcg32 {
        let mut rng = Pcg32 {
            state: 0,
            inc: (seq << 1) | 1
        };
        Rng::<u32>::random(&mut rng);
        rng.state = rng.state.wrapping_add(seed);
        Rng::<u32>::random(&mut rng);
        rng
    }

    pub fn from_current_time_and_address() -> Pcg32 {
        let time = get_current_unix_timestamp();
        let address = &time as *const u64 as u64; // Generally different for each execution of the program.
        //println!("{}, {}", time, address);
        Self::new(time, address)
    }
}

impl Rng<u32> for Pcg32 {
    // Generates a u32 between 0 inclusive and u32::MAX + 1 exclusive.
    fn random(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state.wrapping_mul(6364136223846793005u64) + self.inc;
        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << ((u32::MAX - rot).wrapping_add(1) & 31))
    }

    // Generates a u32 between 0 inclusive and max_exclusive.
    // The range increases from 0 and wraps around the overflow boundary if it needs to.
    fn random_bounded(&mut self, max_exclusive: u32) -> u32 {
        debug_assert!(max_exclusive != 0, "random_bounded: max_exclusive must not be 0");
        let threshold = (u32::MAX - max_exclusive + 1) % max_exclusive;
        loop {
            let r: u32 = self.random();
            if r >= threshold {
                return r % max_exclusive
            }
        }
    }

    // Using a max_exclusive smaller than min_inclusive is perfectly valid, since the range will wrap around the overflow boundary.
    fn random_range(&mut self, min_inclusive: u32, max_exclusive: u32) -> u32 {
        min_inclusive.wrapping_add(self.random_bounded(max_exclusive.wrapping_sub(min_inclusive)))
    }
}

impl Rng<i32> for Pcg32 {
    // Casting between u32 and i32 is a no-op.
    fn random(&mut self) -> i32 {
        Rng::<u32>::random(self) as i32
    }

    // Using a negative argument here will yield results that some would consider surprising.
    // This is because 0 is ALWAYS the LEFT side of the range, and the range is willing to wrap around the overflow boundary.
    fn random_bounded(&mut self, max_exclusive: i32) -> i32 {
        Rng::<u32>::random_bounded(self, max_exclusive as u32) as i32
    }

    // Works just fine with negative inputs!
    fn random_range(&mut self, min_inclusive: i32, max_exclusive: i32) -> i32 {
        Rng::<u32>::random_range(self, min_inclusive as u32, max_exclusive as u32) as i32
    }
}

impl Rng<f32> for Pcg32 {
    // Generates a float between 0.0 inclusive and 1.0 exclusive.
    fn random(&mut self) -> f32 {
        // This distribution is not uniform! Very very small outputs are underrepresented.
        Rng::<u32>::random(self) as f32 * 2f32.powi(-32)
    }

    // Generates a float between 0.0 inclusive and max_exclusive.
    fn random_bounded(&mut self, max_exclusive: f32) -> f32 {
        Rng::<f32>::random(self) * max_exclusive
    }

    fn random_range(&mut self, min_inclusive: f32, max_exclusive: f32) -> f32 {
        min_inclusive + Rng::<f32>::random_bounded(self, max_exclusive - min_inclusive)
    }
}

impl Rng<usize> for Pcg32 {
    fn random(&mut self) -> usize {
        Rng::<u32>::random(self) as usize
    }

    fn random_bounded(&mut self, max_exclusive: usize) -> usize {
        Rng::<u32>::random_bounded(self, max_exclusive as u32) as usize
    }

    fn random_range(&mut self, min_inclusive: usize, max_exclusive: usize) -> usize {
        Rng::<u32>::random_range(self, min_inclusive as u32, max_exclusive as u32) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcg32() {
        let mut rng = Pcg32::from_current_time_and_address();
        let r: u32 = rng.random_range(10, 15);
        assert!(10 <= r && r < 15);
        let r: i32 = rng.random_range(-15, -10);
        assert!(-15 <= r && r < -10);
        let r: i32 = rng.random_range(-15, 30);
        assert!(-15 <= r && r < 30);
        let r: f32 = rng.random_range(10.0, 15.0);
        assert!(10.0 <= r && r < 15.0);
    }
}