use crate::protocol::{ProtocolRead, ProtocolWrite};
use std::io::{self, Error, Read, Write};

use crate::types::Var;

pub mod client;
pub mod server;

// use client::*;
use server::*;

pub trait PacketRead: ProtocolRead + Sized {
    fn packet_read(dst: &mut dyn Read) -> io::Result<Self> {
        Self::proto_decode(dst)
    }
}

pub trait PacketWrite: ProtocolWrite + Sized {
    // fn packet_id() -> i32;
    fn packet_write(&self, dst: &mut dyn Write) -> io::Result<()> {
        let mut r = io::Cursor::new(vec![] as Vec<u8>);

        // <Var<i32>>::proto_encode(&Self::packet_id().into(), &mut r)?;
        Self::proto_encode(self, &mut r)?;
        <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), dst)?;
        dst.write_all(r.get_ref())?;
        dst.flush()
    }
}

pub fn read_handshake_packet(stream: &mut dyn Read) -> Result<HandshakePacket, Error> {
    Ok(HandshakePacket::proto_decode(stream)?)
}

pub fn read_status_packet(stream: &mut dyn Read) -> Result<StatusPacket, Error> {
    Ok(StatusPacket::proto_decode(stream)?)
}

pub fn read_login_packet(stream: &mut dyn Read) -> Result<LoginPacket, Error> {
    Ok(LoginPacket::proto_decode(stream)?)
}

pub fn read_play_packet(stream: &mut dyn Read) -> Result<PlayPacket, Error> {
    Ok(PlayPacket::proto_decode(stream)?)
}
