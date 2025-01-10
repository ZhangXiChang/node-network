use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};

pub trait Borsh
where
    Self: Sized,
{
    fn borsh_from<T: BorshSerialize>(t: &T) -> Result<Self>;
    fn borsh_to<T: BorshDeserialize>(&self) -> Result<T>;
}

impl Borsh for Vec<u8> {
    fn borsh_from<T: BorshSerialize>(t: &T) -> Result<Self> {
        Ok(borsh::to_vec(t)?)
    }
    fn borsh_to<T: BorshDeserialize>(&self) -> Result<T> {
        Ok(borsh::from_slice(self)?)
    }
}
