use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait COBR
where
    Self: Sized,
{
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self>;
    fn message_pack_to<T: DeserializeOwned>(&self) -> Result<T>;
}

impl COBR for Vec<u8> {
    fn message_pack_from<T: Serialize>(value: &T) -> Result<Self> {
        let mut a = Vec::new();
        ciborium::into_writer(value, &mut a)?;
        Ok(a)
    }
    fn message_pack_to<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(ciborium::from_reader(&self[..])?)
    }
}
