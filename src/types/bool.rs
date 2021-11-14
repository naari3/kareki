use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl ProtocolLen for bool {
    fn proto_len(_: &bool) -> usize {
        1
    }
}

impl ProtocolWrite for bool {
    fn proto_encode<D: Write>(value: &bool, dst: &mut D) -> io::Result<()> {
        dst.write_u8(if *value { 1 } else { 0 })?;
        Ok(())
    }
}

impl ProtocolRead for bool {
    fn proto_decode<S: Read>(src: &mut S) -> io::Result<bool> {
        let value = src.read_u8()?;
        if value > 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                &format!("Invalid bool value, expecting 0 or 1, got {}", value)[..],
            ))
        } else {
            Ok(value == 1)
        }
    }
}
