use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::marker::PhantomData;

use num::{NumCast, ToPrimitive};

use crate::protocol::{ProtocolClean, ProtocolLen, ProtocolRead, ProtocolWrite};

pub struct Arr<L, T>(PhantomData<(fn() -> L, T)>);

impl<
        L: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
        T: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
    > ProtocolClean for Arr<L, T>
where
    L::Clean: NumCast,
{
    type Clean = Vec<T::Clean>;
}

impl<
        L: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
        T: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
    > ProtocolLen for Arr<L, T>
where
    L::Clean: NumCast,
{
    fn proto_len(value: &Vec<T::Clean>) -> usize {
        let len_len = <L as ProtocolLen>::proto_len(
            &(<<L as ProtocolClean>::Clean as NumCast>::from(value.len()).unwrap()),
        );
        let len_values = value
            .iter()
            .map(<T as ProtocolLen>::proto_len)
            .fold(0, |acc, item| acc + item);
        len_len + len_values
    }
}

impl<
        L: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
        T: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
    > ProtocolWrite for Arr<L, T>
where
    L::Clean: NumCast,
{
    fn proto_encode(value: &Vec<T::Clean>, dst: &mut dyn Write) -> io::Result<()> {
        let len = <L::Clean as NumCast>::from(value.len()).ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "could not convert length of vector to Array length type",
        ))?;
        <L as ProtocolWrite>::proto_encode(&len, dst)?;
        for elt in value {
            <T as ProtocolWrite>::proto_encode(elt, dst)?;
        }
        Ok(())
    }
}

impl<
        L: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
        T: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite,
    > ProtocolRead for Arr<L, T>
where
    L::Clean: NumCast,
{
    fn proto_decode(src: &mut dyn Read) -> io::Result<Vec<T::Clean>> {
        let len = <L as ProtocolRead>::proto_decode(src)?
            .to_usize()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not read length of vector from Array length type",
            ))?;
        io::Result::from_iter((0..len).map(|_| <T as ProtocolRead>::proto_decode(src)))
    }
}
