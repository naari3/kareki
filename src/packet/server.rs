use std::io::{self, Read};

use crate::{protocol::ProtocolRead, types::Var};

fn read_packet_meta(stream: &mut dyn Read) -> std::io::Result<(u32, u32)> {
    let packet_size = <Var<i32>>::proto_decode(stream)? as u32;
    let packet_id = <Var<i32>>::proto_decode(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);
    Ok((packet_size, packet_id))
}

pub enum HandshakePacket {
    Handshake(Handshake),
}

impl ProtocolRead for HandshakePacket {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Self> {
        let (_packet_size, packet_id) = read_packet_meta(src)?;
        Ok(match packet_id {
            0 => HandshakePacket::Handshake(Handshake::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid HandshakePacket",
                ))
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
}

impl ProtocolRead for Handshake {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            protocol_version: <Var<i32>>::proto_decode(src)?,
            server_address: String::proto_decode(src)?,
            server_port: u16::proto_decode(src)?,
            next_state: NextState::proto_decode(src)?,
        })
    }
}

pub enum StatusPacket {
    Request(Request),
    Ping(Ping),
}

impl ProtocolRead for StatusPacket {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Self> {
        let (_packet_size, packet_id) = read_packet_meta(src)?;
        Ok(match packet_id {
            0 => StatusPacket::Request(Request::proto_decode(src)?),
            1 => StatusPacket::Ping(Ping::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid StatusPacket",
                ))
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct Request {}

impl ProtocolRead for Request {
    fn proto_decode(_src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {})
    }
}

#[derive(Debug, Clone)]
pub struct Ping {
    pub payload: u64,
}

impl ProtocolRead for Ping {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            payload: u64::proto_decode(src)?,
        })
    }
}

pub enum LoginPacket {
    LoginStart {
        name: String,
    },
    EncryptionResponse {
        shared_secret: Vec<u8>,
        verify_token: Vec<u8>,
    },
}

#[derive(Debug, Clone)]
pub struct LoginStart {
    pub name: String,
}

impl ProtocolRead for LoginStart {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            name: String::proto_decode(src)?,
        })
    }
}

pub enum PlayPacket {
    ClientSettings {
        locale: String,
        view_distance: u8,
        chat_mode: i32,
        chat_colors: bool,
        displayed_skin_parts: u8,
        main_hand: i32,
    },
}

#[derive(Debug, Clone)]
pub enum NextState {
    Status,
    Login,
}

impl ProtocolRead for NextState {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(match <Var<i32>>::proto_decode(src)? {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")),
        })
    }
}
