use std::io::Cursor;

use anyhow::Result;
use prost::Message;

pub trait Protobuf {
    fn decode<T: Message + Default>(&self) -> Result<T>;
}

impl Protobuf for Vec<u8> {
    fn decode<T: Message + Default>(&self) -> Result<T> {
        Ok(T::decode(Cursor::new(self))?)
    }
}
