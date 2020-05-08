use std::io;
use std::io::{Read, Write};

pub trait Protocol {
    type Clean;

    fn proto_encode(value: &Self::Clean, dst: &mut dyn Write) -> io::Result<()>;
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self::Clean>;
}