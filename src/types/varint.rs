use crate::protocol::{ProtocolClean, ProtocolLen, ProtocolRead, ProtocolWrite};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};
use std::marker::PhantomData;

pub struct Var<T>(PhantomData<T>);

impl ProtocolClean for Var<i32> {
    type Clean = i32;
}

impl ProtocolLen for Var<i32> {
    /// Size in bytes of `value` as a `Var<i32>`
    fn proto_len(value: &i32) -> usize {
        let value = *value as u32;
        for i in 1..5 {
            if (value & (0xffffffffu32 << (7 * i))) == 0 {
                return i;
            }
        }
        5
    }
}

impl ProtocolWrite for Var<i32> {
    fn proto_encode(value: &i32, dst: &mut dyn Write) -> io::Result<()> {
        let mut temp = *value as u32;
        loop {
            if (temp & !0x7fu32) == 0 {
                dst.write_u8(temp as u8)?;
                return Ok(());
            } else {
                dst.write_u8(((temp & 0x7F) | 0x80) as u8)?;
                temp >>= 7;
            }
        }
    }
}

impl ProtocolRead for Var<i32> {
    fn proto_decode(src: &mut dyn Read) -> io::Result<i32> {
        let mut x = 0i32;

        for shift in (0u32..32).step_by(7).into_iter() {
            let b = src.read_u8()? as i32;
            x |= (b & 0x7F) << shift;
            if (b & 0x80) == 0 {
                return Ok(x);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "VarInt too big",
        ))
    }
}
