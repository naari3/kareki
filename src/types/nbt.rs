use std::{
    fs::File,
    io::{Read, Write},
};

use nbt;
use serde::{de::DeserializeOwned, Serialize};

use crate::protocol::{ProtocolRead, ProtocolWrite};

#[derive(Debug, Clone)]
pub struct Nbt<T>(pub T);

impl<T> ProtocolWrite for Nbt<T>
where
    T: Serialize,
{
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> std::io::Result<()> {
        nbt::to_writer(dst, &value.0, None).unwrap();
        Ok(())
    }
}

impl<T> ProtocolRead for Nbt<T>
where
    T: DeserializeOwned,
{
    fn proto_decode<S: Read>(src: &mut S) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        nbt::from_reader(src).map_err(std::io::Error::from).map(Nbt)
    }
}
