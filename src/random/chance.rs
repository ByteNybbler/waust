use std::fmt::*;
use crate::random::*;

pub struct Chance {
    numerator: u32,
    denominator: u32
}

impl Chance {
    pub fn new(numerator: u32, denominator: u32) -> Chance {
        Chance {
            numerator,
            denominator
        }
    }

    pub fn happens<R>(&self, rng: &mut R) -> bool
    where
        R: Rng<u32>
    {
        rng.random_bounded(self.denominator) < self.numerator
    }

    pub fn get_numerator(&self) -> u32 {
        self.numerator
    }

    pub fn get_denominator(&self) -> u32 {
        self.denominator
    }

    pub fn to_string_fraction(&self) -> String {
        format!("{} in {}", self.numerator, self.denominator)
    }

    pub fn to_string_percent(&self) -> String {
        let ratio = self.numerator as f32 / self.denominator as f32;
        Self::to_string_percent_given_ratio(ratio)
    }

    fn to_string_percent_given_ratio(ratio: f32) -> String {
        format!("{:.0}%", ratio * 100.0)
    }

    pub fn to_string_fraction_or_percent(&self, cutoff: f32) -> String {
        let ratio = self.numerator as f32 / self.denominator as f32;
        if ratio < cutoff {
            self.to_string_fraction()
        } else {
            Self::to_string_percent_given_ratio(ratio)
        }
    }
}

impl Display for Chance {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string_fraction_or_percent(0.01))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chance_display() {
        let chance = Chance::new(3, 4);
        assert_eq!("75%", chance.to_string());
        let chance = Chance::new(1, 100);
        assert_eq!("1%", chance.to_string());
        let chance = Chance::new(1, 99);
        assert_eq!("1%", chance.to_string());
        let chance = Chance::new(4, 200);
        assert_eq!("2%", chance.to_string());
        let chance = Chance::new(3, 200);
        assert_eq!("2%", chance.to_string());
        let chance = Chance::new(5, 200);
        assert_eq!("2%", chance.to_string());
        let chance = Chance::new(1, 101);
        assert_eq!("1 in 101", chance.to_string());
    }

    #[test]
    fn test_chance_happens() {
        let mut rng = Pcg32::from_current_time_and_address();
        assert!(Chance::new(1, 1).happens(&mut rng));
        assert!(!Chance::new(0, 1).happens(&mut rng));
    }
}