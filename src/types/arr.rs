use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::marker::PhantomData;

use num::{NumCast, ToPrimitive};

use crate::protocol::Protocol;

pub struct Arr<L, T>(Vec<T>, PhantomData<(fn() -> L, T)>);

impl<L: Protocol, T: Protocol> Protocol for Arr<L, T>
where
    L::Clean: NumCast + Protocol,
{
    type Clean = Vec<T::Clean>;

    fn proto_len(value: &Vec<T::Clean>) -> usize {
        let len_len = <L as Protocol>::proto_len(
            &(<<L as Protocol>::Clean as NumCast>::from(value.len()).unwrap()),
        );
        let len_values = value
            .iter()
            .map(<T as Protocol>::proto_len)
            .fold(0, |acc, item| acc + item);
        len_len + len_values
    }

    fn proto_encode(&self, dst: &mut dyn Write) -> io::Result<()> {
        let len = <L::Clean as NumCast>::from(self.0.len()).ok_or(io::Error::new(
            io::ErrorKind::InvalidInput,
            "could not convert length of vector to Array length type",
        ))?;
        len.proto_encode(dst)?;
        for elt in self.0 {
            elt.proto_encode(dst)?;
        }
        Ok(())
    }

    fn proto_decode(src: &mut dyn Read) -> io::Result<Vec<T::Clean>> {
        let len = <L as Protocol>::proto_decode(src)?
            .to_usize()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not read length of vector from Array length type",
            ))?;
        io::Result::from_iter((0..len).map(|_| <T as Protocol>::proto_decode(src)))
    }
}
