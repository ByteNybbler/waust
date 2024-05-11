use std::marker::PhantomData;

use serde::{de::{self, SeqAccess, Visitor}, Deserialize, Deserializer, Serialize};

/// A Vec that gets serialized in a special way by serde.
#[derive(Debug, Serialize)]
pub struct VecLen<T>(pub Vec<T>);

pub struct VecLenVisitor<T> {
    phantom: PhantomData<T>
}

impl<'de, T> Deserialize<'de> for VecLen<T>
where
    T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<VecLen<T>, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_struct("VecLen", &["0"], VecLenVisitor::new())
    }
}

impl<T> VecLenVisitor<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for VecLenVisitor<T>
where
    T: Deserialize<'de>
{
    type Value = VecLen<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "struct VecLen")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<VecLen<T>, V::Error>
    where
        V: SeqAccess<'de>
    {
        let length: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let mut vector = Vec::with_capacity(length as usize);
        for _ in 0..length as usize {
            let element = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            vector.push(element);
        }
        Ok(VecLen(vector))
    }
}