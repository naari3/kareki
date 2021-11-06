use std::io;
use std::io::{Read, Write};
use std::str::FromStr;

use uuid::Uuid;

use super::varint::Var;
use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

impl ProtocolLen for Uuid {
    fn proto_len(value: &Self) -> usize {
        String::proto_len(&value.to_string())
    }
}

impl ProtocolWrite for Uuid {
    fn proto_encode(value: &Uuid, dst: &mut dyn Write) -> io::Result<()> {
        String::proto_encode(&value.to_string(), dst)
    }
}

impl ProtocolRead for Uuid {
    fn proto_decode(src: &mut dyn Read) -> io::Result<Uuid> {
        let value = String::proto_decode(src)?;
        Ok(Uuid::from_str(&value).unwrap())
    }
}
