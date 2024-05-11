use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn splat(value: T) -> Self
    where
        T: Copy
    {
        Self::new(value, value)
    }
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn splat(value: T) -> Self
    where
        T: Copy
    {
        Self::new(value, value, value)
    }
}