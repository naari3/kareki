use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

pub fn encode_varint(value: &i32, dst: &mut dyn Write) -> io::Result<()> {
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

pub fn decode_varint(src: &mut dyn Read) -> io::Result<i32> {
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