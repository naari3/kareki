use std::io::Write;

use nbt;
use serde::Serialize;

use crate::protocol::ProtocolWrite;

#[derive(Debug, Clone)]
pub struct Nbt<T>(pub T);
impl<T> ProtocolWrite for Nbt<T>
where
    T: Serialize,
{
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> std::io::Result<()> {
        nbt::to_writer(dst, &value.0, None).unwrap();
        Ok(())
    }
}
