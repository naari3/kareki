use std::io::{Read, Result, Write};

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}
impl ProtocolLen for Position {
    fn proto_len(_value: &Self) -> usize {
        u64::proto_len(&(0 as _))
    }
}

impl ProtocolWrite for Position {
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> Result<()> {
        u64::proto_encode(
            &((((value.x as u64) & 0x3FFFFFF) << 38)
                | (((value.z as u64) & 0x3FFFFFF) << 12)
                | ((value.y as u64) & 0xFFF)),
            dst,
        )
    }
}

impl ProtocolRead for Position {
    fn proto_decode<S: Read>(src: &mut S) -> std::io::Result<Self> {
        let datum = i64::proto_decode(src)?;
        Ok(Self {
            x: (datum >> 38) as i32,
            y: (datum & 0xFFF) as i16,
            z: (datum << 26 >> 38) as i32,
        })
    }
}
