use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::marker::PhantomData;

use num::ToPrimitive;

use crate::protocol::{ProtocolLen, ProtocolRead, ProtocolWrite};

use super::Var;

pub struct Arr<L, T>(PhantomData<(fn() -> L, T)>);

impl<T: ProtocolLen> ProtocolLen<Vec<T>> for Arr<Var<i32>, T> {
    fn proto_len(value: &Vec<T>) -> usize {
        let len_len = <Var<i32>>::proto_len(&((value.len() as i32).into()));
        let len_values = value
            .iter()
            .map(<T as ProtocolLen>::proto_len)
            .fold(0, |acc, item| acc + item);
        len_len + len_values
    }
}

impl<T: ProtocolWrite> ProtocolWrite<Vec<T>> for Arr<Var<i32>, T> {
    fn proto_encode<D: Write>(value: &Vec<T>, dst: &mut D) -> io::Result<()> {
        let len = (value.len() as i32).into();
        <Var<i32>>::proto_encode(&len, dst)?;
        for elt in value {
            <T as ProtocolWrite>::proto_encode(elt, dst)?;
        }
        Ok(())
    }
}

impl<T: ProtocolRead> ProtocolRead<Vec<T>> for Arr<Var<i32>, T> {
    fn proto_decode<S: Read>(src: &mut S) -> io::Result<Vec<T>> {
        let len = i32::from(<Var<i32>>::proto_decode(src)?)
            .to_usize()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidInput,
                "could not read length of vector from Array length type",
            ))?;
        io::Result::from_iter((0..len).map(|_| <T as ProtocolRead>::proto_decode(src)))
    }
}
