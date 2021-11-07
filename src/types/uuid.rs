use std::io;
use std::io::{Read, Write};

use uuid::Uuid;

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl ProtocolLen for Uuid {
    fn proto_len(value: &Self) -> usize {
        String::proto_len(&value.to_string())
    }
}

impl ProtocolWrite for Uuid {
    fn proto_encode(value: &Uuid, dst: &mut dyn Write) -> io::Result<()> {
        for b in value.as_bytes() {
            u8::proto_encode(b, dst)?;
        }
        Ok(())
    }
}

impl ProtocolRead for Uuid {
    fn proto_decode(src: &mut dyn Read) -> io::Result<Uuid> {
        let mut bytes = Vec::new();
        for _ in 0..16 {
            bytes.push(u8::proto_decode(src)?);
        }
        let uuid = Uuid::from_slice(&bytes).unwrap();
        Ok(uuid)
    }
}
