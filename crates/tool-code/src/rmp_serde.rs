use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait MessagePack<'a>
where
    Self: Sized,
{
    fn encode<T: Serialize>(value: &T) -> Result<Self>;
    fn decode<T: Deserialize<'a>>(&'a self) -> Result<T>;
}
impl<'a> MessagePack<'a> for Vec<u8> {
    fn encode<T: Serialize>(value: &T) -> Result<Self> {
        Ok(rmp_serde::to_vec(value)?)
    }
    fn decode<T: Deserialize<'a>>(&'a self) -> Result<T> {
        Ok(rmp_serde::from_slice::<T>(self)?)
    }
}
