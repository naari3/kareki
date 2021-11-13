use kareki_macros::ProtocolWrite;

use std::io::Write;

#[derive(ProtocolWrite)]
#[packet_id = 0]
pub struct Yoyo {
    gogo: u64,
}

pub struct Var<T>(pub T);
impl From<i32> for Var<i32> {
    fn from(n: i32) -> Self {
        Self(n)
    }
}

trait ProtocolWrite<Clean = Self> {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> std::io::Result<()>;
}

impl ProtocolWrite for u64 {
    fn proto_encode(_value: &Self, _dst: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }
}

impl ProtocolWrite for Var<i32> {
    fn proto_encode(_value: &Self, _dst: &mut dyn Write) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let src = vec![0; 8];
    let mut cursor = std::io::Cursor::new(src);
    let _ = Yoyo::proto_encode(&Yoyo { gogo: 0u64 }, &mut cursor).unwrap();
}
