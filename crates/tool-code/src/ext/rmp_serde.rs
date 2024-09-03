use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait MessagePack<'a>
where
    Self: Sized,
{
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self>;
    fn message_pack_to<T: Deserialize<'a>>(&'a self) -> Result<T>;
}
impl<'a> MessagePack<'a> for Vec<u8> {
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self> {
        Ok(rmp_serde::to_vec(value)?)
    }
    fn message_pack_to<T: Deserialize<'a>>(&'a self) -> Result<T> {
        Ok(rmp_serde::from_slice::<T>(self)?)
    }
}
