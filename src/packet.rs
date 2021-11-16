use crate::protocol::{ProtocolRead, ProtocolWrite};
use std::io::{self, Error, Read, Write};

use crate::types::Var;

pub mod client;
pub mod server;

// use client::*;
use server::*;

pub trait PacketRead: ProtocolRead + Sized {
    fn packet_read<D: Read>(src: &mut D) -> io::Result<Self> {
        Self::proto_decode(src)
    }
}

pub trait PacketWrite: ProtocolWrite + Sized {
    fn packet_id() -> i32;
    fn packet_write<D: Write>(&self, dst: &mut D) -> io::Result<()> {
        let mut r = io::Cursor::new(vec![] as Vec<u8>);

        <Var<i32>>::proto_encode(&Self::packet_id().into(), &mut r)?;
        Self::proto_encode(self, &mut r)?;
        <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), dst)?;
        dst.write_all(r.get_ref())?;
        dst.flush()
    }
}

pub trait PacketWriteEnum {
    fn packet_write<D: Write>(&self, dst: &mut D) -> io::Result<()>;
}

pub trait PacketReadEnum: Sized {
    fn packet_read<S: std::io::Read>(src: &mut S) -> std::io::Result<Self>;
}

pub fn read_handshake_packet<D: Read>(src: &mut D) -> Result<HandshakePacket, Error> {
    Ok(HandshakePacket::packet_read(src)?)
}

pub fn read_status_packet<D: Read>(src: &mut D) -> Result<StatusPacket, Error> {
    Ok(StatusPacket::packet_read(src)?)
}

pub fn read_login_packet<D: Read>(src: &mut D) -> Result<LoginPacket, Error> {
    Ok(LoginPacket::packet_read(src)?)
}

pub fn read_play_packet<D: Read>(src: &mut D) -> Result<PlayPacket, Error> {
    Ok(PlayPacket::packet_read(src)?)
}
