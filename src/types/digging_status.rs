use std::io::{Error, ErrorKind, Read, Result, Write};

use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

use crate::protocol::{ProtocolRead, ProtocolWrite};

use super::Var;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum DiggingStatus {
    StartedDigging,
    CancelledDigging,
    FinishedDigging,
    DropItemStack,
    DropItem,
    ShootArrowOrFinishEating,
    SwapItemInHand,
}

impl ProtocolRead for DiggingStatus {
    fn proto_decode<S: Read>(src: &mut S) -> Result<Self> {
        let face = <Var<i32>>::proto_decode(src)?;
        Self::from_i32(face.0).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "could not use number other than 0 to 5",
        ))
    }
}

impl ProtocolWrite for DiggingStatus {
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> Result<()> {
        let face = match value.to_i32() {
            Some(n) => Var(n),
            None => panic!("wtf i don't think reach this line"),
        };
        <Var<i32>>::proto_encode(&face, dst)
    }
}
