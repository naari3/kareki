use std::io::{Error, ErrorKind, Read, Result, Write};

use crate::protocol::{ProtocolRead, ProtocolWrite};
use num::{FromPrimitive, ToPrimitive};
use num_derive::{FromPrimitive, ToPrimitive};

use super::Var;

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum BlockFaceU8 {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}

impl ProtocolRead for BlockFace {
    fn proto_decode<S: Read>(src: &mut S) -> Result<Self> {
        let face = <Var<i32>>::proto_decode(src)?;
        BlockFace::from_i32(face.0).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "could not use number other than 0 to 5",
        ))
    }
}

impl ProtocolWrite for BlockFace {
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> Result<()> {
        let face = match value.to_i32() {
            Some(n) => Var(n),
            None => panic!("wtf i don't think reach this line"),
        };
        <Var<i32>>::proto_encode(&face, dst)
    }
}

impl ProtocolRead for BlockFaceU8 {
    fn proto_decode<S: Read>(src: &mut S) -> Result<Self> {
        let face = u8::proto_decode(src)?;
        BlockFaceU8::from_u8(face).ok_or(Error::new(
            ErrorKind::InvalidInput,
            "could not use number other than 0 to 5",
        ))
    }
}

impl ProtocolWrite for BlockFaceU8 {
    fn proto_encode<D: Write>(value: &Self, dst: &mut D) -> Result<()> {
        let face = match value.to_u8() {
            Some(n) => n,
            None => panic!("wtf i don't think reach this line"),
        };
        u8::proto_encode(&face, dst)
    }
}
