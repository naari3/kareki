use crate::protocol::{ProtocolRead, ProtocolWrite};
use std::io::{self, Error, Read, Write};

use crate::types::{Arr, Var};

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
    fn packet_write(&self, dst: &mut dyn Write) -> io::Result<()> {
        let mut r = io::Cursor::new(vec![] as Vec<u8>);

        Self::proto_encode(self, &mut r)?;
        <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), dst)?;
        dst.write_all(r.get_ref())?;
        dst.flush()
    }
}

fn read_packet_meta(stream: &mut dyn Read) -> Result<(u32, u32), Error> {
    let packet_size = <Var<i32>>::proto_decode(stream)? as u32;
    let packet_id = <Var<i32>>::proto_decode(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);
    Ok((packet_size, packet_id))
}

pub fn read_handshake_packet(stream: &mut dyn Read) -> Result<HandshakePacket, Error> {
    println!("get handshake");
    Ok(HandshakePacket::proto_decode(stream)?)
}

pub fn read_status_packet(stream: &mut dyn Read) -> Result<StatusPacket, Error> {
    println!("get status");
    Ok(StatusPacket::proto_decode(stream)?)
}

pub fn read_login_packet(stream: &mut dyn Read) -> Result<LoginPacket, Error> {
    let (_, packet_id) = read_packet_meta(stream)?;

    match packet_id {
        0 => {
            return Ok(LoginPacket::LoginStart {
                name: String::proto_decode(stream)?,
            })
        }
        1 => {
            let shared_secret = <Arr<Var<i32>, u8>>::proto_decode(stream)?;
            let verify_token = <Arr<Var<i32>, u8>>::proto_decode(stream)?;

            return Ok(LoginPacket::EncryptionResponse {
                shared_secret,
                verify_token,
            });
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }
}

pub fn read_play_packet(stream: &mut dyn Read) -> Result<PlayPacket, Error> {
    let (_, packet_id) = read_packet_meta(stream)?;

    match packet_id {
        5 => {
            return Ok(PlayPacket::ClientSettings {
                locale: String::proto_decode(stream)?,
                view_distance: u8::proto_decode(stream)?,
                chat_mode: <Var<i32>>::proto_decode(stream)?,
                chat_colors: bool::proto_decode(stream)?,
                displayed_skin_parts: u8::proto_decode(stream)?,
                main_hand: <Var<i32>>::proto_decode(stream)?,
            })
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }
}
