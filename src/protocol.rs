use std::io;
use std::io::{Read, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait Protocol {
    type Clean;

    fn proto_len(value: &Self::Clean) -> usize;
    fn proto_encode(value: &Self::Clean, dst: &mut dyn Write) -> io::Result<()>;
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self::Clean>;
}

impl Protocol for u8 {
    type Clean = Self;

    fn proto_len(_: &u8) -> usize { 1 }

    fn proto_encode(value: &u8, dst: &mut dyn Write) -> io::Result<()> {
        dst.write_u8(*value)?;
        Ok(())
    }

    fn proto_decode(src: &mut dyn Read) -> io::Result<u8> {
        src.read_u8().map_err(|err| io::Error::from(err))
    }
}