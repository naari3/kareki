use std::io::{self, Read, Write};

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl<T> ProtocolLen for Option<T>
where
    T: ProtocolLen,
{
    fn proto_len(value: &Self) -> usize {
        match value {
            Some(v) => T::proto_len(v),
            None => 0,
        }
    }
}

impl<T> ProtocolWrite for Option<T>
where
    T: ProtocolWrite,
{
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> io::Result<()> {
        match value {
            Some(v) => T::proto_encode(v, dst)?,
            None => {}
        }
        Ok(())
    }
}

// Maybe has bug
impl<T> ProtocolRead for Option<T>
where
    T: ProtocolRead,
{
    fn proto_decode<S: Read>(src: &mut S) -> io::Result<Self> {
        let value = T::proto_decode(src)?;
        Ok(Some(value))
    }
}
