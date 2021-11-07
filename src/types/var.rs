use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::Display;
use std::io;
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Var<T>(T);

impl From<i32> for Var<i32> {
    fn from(n: i32) -> Self {
        Self(n)
    }
}

impl From<Var<i32>> for i32 {
    fn from(v: Var<i32>) -> Self {
        v.0
    }
}

impl From<i64> for Var<i64> {
    fn from(n: i64) -> Self {
        Self(n)
    }
}

impl From<Var<i64>> for i64 {
    fn from(v: Var<i64>) -> Self {
        v.0
    }
}

impl<T> Display for Var<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ProtocolLen for Var<i32> {
    /// Size in bytes of `value` as a `Var<i32>`
    fn proto_len(value: &Self) -> usize {
        let value = value.0 as u32;
        for i in 1..5 {
            if (value & (0xffffffffu32 << (7 * i))) == 0 {
                return i;
            }
        }
        5
    }
}

impl ProtocolWrite for Var<i32> {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        let mut temp = value.0 as u32;
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
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self> {
        let mut x = 0i32;

        for shift in (0u32..32).step_by(7).into_iter() {
            let b = src.read_u8()? as i32;
            x |= (b & 0x7F) << shift;
            if (b & 0x80) == 0 {
                return Ok(Var(x));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "VarInt too big",
        ))
    }
}

impl ProtocolLen for Var<i64> {
    /// Size in bytes of `value` as a `Var<i32>`
    fn proto_len(value: &Self) -> usize {
        let value = value.0 as u64;
        for i in 1..5 {
            if (value & (0xffffffffu64 << (7 * i))) == 0 {
                return i;
            }
        }
        5
    }
}

impl ProtocolWrite for Var<i64> {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        let mut temp = value.0 as u64;
        loop {
            if (temp & !0x7fu64) == 0 {
                dst.write_u8(temp as u8)?;
                return Ok(());
            } else {
                dst.write_u8(((temp & 0x7F) | 0x80) as u8)?;
                temp >>= 7;
            }
        }
    }
}

impl ProtocolRead for Var<i64> {
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self> {
        let mut x = 0i64;

        for shift in (0u64..32).step_by(7).into_iter() {
            let b = src.read_u8()? as i64;
            x |= (b & 0x7F) << shift;
            if (b & 0x80) == 0 {
                return Ok(Var(x));
            }
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "VarInt too big",
        ))
    }
}
