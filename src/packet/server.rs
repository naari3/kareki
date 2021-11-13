use std::io::{self, Read};

use kareki_macros::ProtocolRead;

use crate::{
    protocol::ProtocolRead,
    types::{position::Position, Arr, Var},
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

#[derive(Debug, Clone, ProtocolRead)]
pub struct Handshake {
    pub protocol_version: Var<i32>,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: NextState,
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

#[derive(Debug, Clone, ProtocolRead)]
pub struct Request {}

#[derive(Debug, Clone, ProtocolRead)]
pub struct Ping {
    pub payload: u64,
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

#[derive(Debug, Clone, ProtocolRead)]
pub struct LoginStart {
    pub name: String,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

pub enum PlayPacket {
    /* 0x05 */ ClientSettings(ClientSettings),
    /* 0x0F */ KeepAlive(KeepAlive),
    /* 0x11 */ PlayerPosition(PlayerPosition),
    /* 0x12 */ PlayerPositionAndRotation(PlayerPositionAndRotation),
    /* 0x2C */ PlayerBlockPlacement(PlayerBlockPlacement),
}

impl ProtocolRead for PlayPacket {
    fn proto_decode(src: &mut dyn Read) -> std::io::Result<Self> {
        let (_packet_size, packet_id) = read_packet_meta(src)?;
        Ok(match packet_id {
            0x05 => PlayPacket::ClientSettings(ClientSettings::proto_decode(src)?),
            0x0F => PlayPacket::KeepAlive(KeepAlive::proto_decode(src)?),
            0x11 => PlayPacket::PlayerPosition(PlayerPosition::proto_decode(src)?),
            0x12 => {
                PlayPacket::PlayerPositionAndRotation(PlayerPositionAndRotation::proto_decode(src)?)
            }
            0x2C => PlayPacket::PlayerBlockPlacement(PlayerBlockPlacement::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "invalid PlayPacket",
                ))
            }
        })
    }
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct ClientSettings {
    pub locale: String,
    pub view_distance: u8,
    pub chat_mode: Var<i32>,
    pub chat_colors: bool,
    pub displayed_skin_parts: u8,
    pub main_hand: Var<i32>,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct KeepAlive {
    pub id: i64,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerPositionAndRotation {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerBlockPlacement {
    pub hand: Var<i32>,
    pub location: Position,
    pub face: Var<i32>,
    pub cursor_point_x: f32,
    pub cursor_point_y: f32,
    pub cursor_point_z: f32,
    pub inside_block: bool,
}
