use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io;
use std::io::{Read, Write};

pub trait Protocol: ProtocolClean + ProtocolLen + ProtocolRead + ProtocolWrite {
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self::Clean>;
    fn proto_encode(value: &Self::Clean, dst: &mut dyn Write) -> io::Result<()>;
    fn proto_len(value: &Self::Clean) -> usize;
}

pub trait ProtocolClean {
    type Clean;
}

pub trait ProtocolRead: ProtocolClean {
    fn proto_decode(src: &mut dyn Read) -> io::Result<Self::Clean>;
}

pub trait ProtocolWrite: ProtocolClean {
    fn proto_encode(value: &Self::Clean, dst: &mut dyn Write) -> io::Result<()>;
}

pub trait ProtocolLen: ProtocolClean {
    fn proto_len(value: &Self::Clean) -> usize;
}

macro_rules! impl_protocol {
    ($name:ty, 1, $enc_name:ident, $dec_name:ident) => {
        impl ProtocolClean for $name {
            type Clean = Self;
        }

        impl ProtocolLen for $name {
            fn proto_len(_: &$name) -> usize {
                1
            }
        }
        impl ProtocolWrite for $name {
            fn proto_encode(value: &$name, dst: &mut dyn Write) -> io::Result<()> {
                dst.$enc_name(*value)?;
                Ok(())
            }
        }
        impl ProtocolRead for $name {
            fn proto_decode(src: &mut dyn Read) -> io::Result<$name> {
                src.$dec_name().map_err(|err| io::Error::from(err))
            }
        }
    };
    ($name:ty, $len:expr, $enc_name:ident, $dec_name:ident) => {
        impl ProtocolClean for $name {
            type Clean = Self;
        }

        impl ProtocolLen for $name {
            fn proto_len(_: &$name) -> usize {
                $len
            }
        }

        impl ProtocolWrite for $name {
            fn proto_encode(value: &$name, dst: &mut dyn Write) -> io::Result<()> {
                dst.$enc_name::<BigEndian>(*value)?;
                Ok(())
            }
        }

        impl ProtocolRead for $name {
            fn proto_decode(src: &mut dyn Read) -> io::Result<$name> {
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

impl ProtocolClean for bool {
    type Clean = bool;
}

impl ProtocolLen for bool {
    fn proto_len(_: &bool) -> usize {
        1
    }
}
impl ProtocolWrite for bool {
    fn proto_encode(value: &bool, dst: &mut dyn Write) -> io::Result<()> {
        dst.write_u8(if *value { 1 } else { 0 })?;
        Ok(())
    }
}

impl ProtocolRead for bool {
    fn proto_decode(src: &mut dyn Read) -> io::Result<bool> {
        let value = src.read_u8()?;
        if value > 1 {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                &format!("Invalid bool value, expecting 0 or 1, got {}", value)[..],
            ))
        } else {
            Ok(value == 1)
        }
    }
}
