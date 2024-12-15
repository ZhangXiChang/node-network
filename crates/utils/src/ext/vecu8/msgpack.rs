use anyhow::Result;
use serde::{Deserialize, Serialize};

pub trait MessagePack
where
    Self: Sized,
{
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self>;
    fn message_pack_to<'a, T: Deserialize<'a>>(&'a self) -> Result<T>;
}

impl MessagePack for Vec<u8> {
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self> {
        Ok(rmp_serde::to_vec(value)?)
    }
    fn message_pack_to<'a, T: Deserialize<'a>>(&'a self) -> Result<T> {
        Ok(rmp_serde::from_slice::<T>(self)?)
    }
}
