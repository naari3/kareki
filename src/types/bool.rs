use std::io::{self, Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl ProtocolLen for bool {
    fn proto_len(_: &bool) -> usize {
        1
    }
}

impl ProtocolWrite for bool {
    fn proto_encode(value: &bool, dst: &mut dyn Write) -> io::Result<()> {
        dst.write_u8(if *value { 1 } else { 0 })?;
        Ok(())
    }
}

impl ProtocolRead for bool {
    fn proto_decode(src: &mut dyn Read) -> io::Result<bool> {
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
