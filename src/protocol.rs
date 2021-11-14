use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

pub trait ProtocolRead<Clean = Self> {
    fn proto_decode<S: Read>(src: &mut S) -> io::Result<Clean>;
}

pub trait ProtocolWrite<Clean = Self> {
    fn proto_encode<D: Write>(value: &Clean, dst: &mut D) -> io::Result<()>;
}

pub trait ProtocolLen<Clean = Self> {
    fn proto_len(value: &Clean) -> usize;
}

macro_rules! impl_protocol {
    ($name:ty, 1, $enc_name:ident, $dec_name:ident) => {
        impl ProtocolLen for $name {
            fn proto_len(_: &$name) -> usize {
                1
            }
        }
        impl ProtocolWrite for $name {
            fn proto_encode<D: Write>(value: &$name, dst: &mut D) -> io::Result<()> {
                dst.$enc_name(*value)?;
                Ok(())
            }
        }
        impl ProtocolRead for $name {
            fn proto_decode<S: Read>(src: &mut S) -> io::Result<$name> {
                src.$dec_name().map_err(|err| io::Error::from(err))
            }
        }
    };
    ($name:ty, $len:expr, $enc_name:ident, $dec_name:ident) => {
        impl ProtocolLen for $name {
            fn proto_len(_: &$name) -> usize {
                $len
            }
        }

        impl ProtocolWrite for $name {
            fn proto_encode<D: Write>(value: &$name, dst: &mut D) -> io::Result<()> {
                dst.$enc_name::<BigEndian>(*value)?;
                Ok(())
            }
        }

        impl ProtocolRead for $name {
            fn proto_decode<S: Read>(src: &mut S) -> io::Result<$name> {
                src.$dec_name::<BigEndian>()
                    .map_err(|err| io::Error::from(err))
            }
        }
    };
}

impl_protocol!(i8, 1, write_i8, read_i8);
impl_protocol!(u8, 1, write_u8, read_u8);
impl_protocol!(i16, 2, write_i16, read_i16);
impl_protocol!(u16, 2, write_u16, read_u16);
impl_protocol!(i32, 4, write_i32, read_i32);
impl_protocol!(u32, 4, write_u32, read_u32);
impl_protocol!(i64, 8, write_i64, read_i64);
impl_protocol!(u64, 8, write_u64, read_u64);
impl_protocol!(f32, 4, write_f32, read_f32);
impl_protocol!(f64, 8, write_f64, read_f64);
