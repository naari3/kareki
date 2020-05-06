use std::io;
use std::io::{Read, Write};

use super::varint::{decode_varint, encode_varint};

pub fn encode_string(value: &String, dst: &mut dyn Write) -> io::Result<()> {
    let str_len = value.len() as i32;
    encode_varint(&str_len, dst)?;
    dst.write_all(value.as_bytes())?;
    Ok(())
}

pub fn decode_string(src: &mut dyn Read) -> io::Result<String> {
    let len: i32 = decode_varint(src)?;
    let mut s = vec![0u8; len as usize];
    src.read_exact(&mut s)?;
    String::from_utf8(s).map_err(|utf8_err| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            &format!("UTF-8 error: {:?}", utf8_err.utf8_error())[..],
        )
    })
}
