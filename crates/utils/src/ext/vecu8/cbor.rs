use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait COBR
where
    Self: Sized,
{
    fn cbor_from<T: Serialize>(value: &T) -> Result<Self>;
    fn cbor_to<T: DeserializeOwned>(&self) -> Result<T>;
}

impl COBR for Vec<u8> {
    fn cbor_from<T: Serialize>(value: &T) -> Result<Self> {
        let mut a = Vec::new();
        ciborium::into_writer(value, &mut a)?;
        Ok(a)
    }
    fn cbor_to<T: DeserializeOwned>(&self) -> Result<T> {
        Ok(ciborium::from_reader(&self[..])?)
    }
}
