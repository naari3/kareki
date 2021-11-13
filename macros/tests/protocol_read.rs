use kareki_macros::ProtocolRead;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

#[derive(ProtocolRead)]
pub struct Gogo {
    yoyo: u64,
}

trait ProtocolRead<Clean = Self> {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Clean>;
}

impl ProtocolRead for u64 {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<u64> {
        src.read_u64::<BigEndian>()
            .map_err(|err| std::io::Error::from(err))
    }
}

fn main() {
    let src = vec![0; 8];
    let mut cursor = std::io::Cursor::new(src);
    let _ = Gogo::proto_decode(&mut cursor).unwrap();
}
