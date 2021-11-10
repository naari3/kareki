use std::io::{self, Read};

use crate::{
    protocol::ProtocolRead,
    types::{Arr, Var},
};

fn read_packet_meta(stream: &mut dyn Read) -> std::io::Result<(u32, u32)> {
    let packet_size = i32::from(<Var<i32>>::proto_decode(stream)?) as u32;
    let packet_id = i32::from(<Var<i32>>::proto_decode(stream)?) as u32;
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
    pub protocol_version: Var<i32>,
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

#[derive(Debug, Clone)]
pub enum NextState {
    Status,
    Login,
}

impl ProtocolRead for NextState {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(match <Var<i32>>::proto_decode(src)?.into() {
            1 => NextState::Status,
            2 => NextState::Login,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")),
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
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
}

impl ProtocolRead for LoginPacket {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Self> {
        let (_packet_size, packet_id) = read_packet_meta(src)?;
        Ok(match packet_id {
            0 => LoginPacket::LoginStart(LoginStart::proto_decode(src)?),
            1 => LoginPacket::EncryptionResponse(EncryptionResponse::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid LoginPacket",
                ))
            }
        })
    }
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

#[derive(Debug, Clone)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl ProtocolRead for EncryptionResponse {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            shared_secret: <Arr<Var<i32>, u8>>::proto_decode(src)?,
            verify_token: <Arr<Var<i32>, u8>>::proto_decode(src)?,
        })
    }
}

pub enum PlayPacket {
    ClientSettings(ClientSettings),
    KeepAlive(KeepAlive),
}

impl ProtocolRead for PlayPacket {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Self> {
        let (_packet_size, packet_id) = read_packet_meta(src)?;
        Ok(match packet_id {
            0x05 => PlayPacket::ClientSettings(ClientSettings::proto_decode(src)?),
            0x0F => PlayPacket::KeepAlive(KeepAlive::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid PlayPacket",
                ))
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: Var<i32>,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: Var<i32>,
}

impl ProtocolRead for ClientSettings {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            locale: String::proto_decode(src)?,
            view_distance: u8::proto_decode(src)?,
            chat_mode: <Var<i32>>::proto_decode(src)?,
            chat_colors: bool::proto_decode(src)?,
            displayed_skin_parts: u8::proto_decode(src)?,
            main_hand: <Var<i32>>::proto_decode(src)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlive {
    pub id: i64,
}

impl ProtocolRead for KeepAlive {
    fn proto_decode(src: &mut dyn std::io::Read) -> std::io::Result<Self> {
        Ok(Self {
            id: i64::proto_decode(src)?,
        })
    }
}
