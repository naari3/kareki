use std::io;
use std::io::{Read, Write};

use super::varint::Var;
use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl ProtocolLen for String {
    fn proto_len(value: &String) -> usize {
        let str_len = value.len();
        <Var<i32>>::proto_len(&(str_len as i32).into()) + str_len
    }
}

impl ProtocolWrite for String {
    fn proto_encode(value: &String, dst: &mut dyn Write) -> io::Result<()> {
        let str_len = value.len() as i32;
        <Var<i32>>::proto_encode(&str_len.into(), dst)?;
        dst.write_all(value.as_bytes())?;
        Ok(())
    }
}

impl ProtocolRead for String {
    fn proto_decode(src: &mut dyn Read) -> io::Result<String> {
        let len: i32 = <Var<i32>>::proto_decode(src)?.into();
        let mut s = vec![0u8; len as usize];
        src.read_exact(&mut s)?;
        String::from_utf8(s).map_err(|utf8_err| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                &format!("UTF-8 error: {:?}", utf8_err.utf8_error())[..],
            )
        })
    }
}
