use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorRgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}

impl<T> ColorRgb<T> {
    pub const fn new(red: T, green: T, blue: T) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }

    pub const fn splat(value: T) -> Self
    where
        T: Copy
    {
        Self::new(value, value, value)
    }
}

impl ColorRgb<i32> {
    pub const DEFAULT_LIGHT: Self = Self::splat(255);
    pub const DEFAULT_AMBIENT: Self = Self::splat(100);
}