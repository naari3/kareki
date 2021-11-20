use std::io::{self, Read};

use kareki_macros::ProtocolRead;

use crate::{
    protocol::ProtocolRead,
    types::{
        block_face::{BlockFace, BlockFaceU8},
        digging_status::DiggingStatus,
        position::Position,
        slot::Slot,
        Arr, Var,
    },
};

use super::PacketReadEnum;

fn read_packet_meta<S: Read>(src: &mut S) -> std::io::Result<u32> {
    let packet_id = i32::from(<Var<i32>>::proto_decode(src)?) as u32;
    Ok(packet_id)
}

pub enum HandshakePacket {
    Handshake(Handshake),
}

impl PacketReadEnum for HandshakePacket {
    fn packet_read<S: Read>(src: &mut S) -> std::io::Result<Self> {
        let packet_id = read_packet_meta(src)?;
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
    fn proto_decode<S: std::io::Read>(src: &mut S) -> std::io::Result<Self> {
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

impl PacketReadEnum for StatusPacket {
    fn packet_read<S: Read>(src: &mut S) -> std::io::Result<Self> {
        let packet_id = read_packet_meta(src)?;
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

impl PacketReadEnum for LoginPacket {
    fn packet_read<S: Read>(src: &mut S) -> std::io::Result<Self> {
        let packet_id = read_packet_meta(src)?;
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

#[derive(Debug, Clone)]
pub enum PlayPacket {
    /* 0x00 */ TeleportConfirm(TeleportConfirm),
    /* 0x05 */ ClientSettings(ClientSettings),
    /* 0x0F */ KeepAlive(KeepAlive),
    /* 0x11 */ PlayerPosition(PlayerPosition),
    /* 0x12 */ PlayerPositionAndRotation(PlayerPositionAndRotation),
    /* 0x13 */ PlayerRotation(PlayerRotation),
    /* 0x1A */ PlayerAbilities(PlayerAbilities),
    /* 0x1A */ PlayerDigging(PlayerDigging),
    /* 0x1B */ EntityAction(EntityAction),
    /* 0x23 */ HeldItemChange(HeldItemChange),
    /* 0x26 */ CreativeInventoryAction(CreativeInventoryAction),
    /* 0x2C */ PlayerBlockPlacement(PlayerBlockPlacement),
}

impl PacketReadEnum for PlayPacket {
    fn packet_read<S: Read>(src: &mut S) -> std::io::Result<Self> {
        let packet_id = read_packet_meta(src)?;
        Ok(match packet_id {
            0x00 => PlayPacket::TeleportConfirm(TeleportConfirm::proto_decode(src)?),
            0x05 => PlayPacket::ClientSettings(ClientSettings::proto_decode(src)?),
            0x0F => PlayPacket::KeepAlive(KeepAlive::proto_decode(src)?),
            0x11 => PlayPacket::PlayerPosition(PlayerPosition::proto_decode(src)?),
            0x12 => {
                PlayPacket::PlayerPositionAndRotation(PlayerPositionAndRotation::proto_decode(src)?)
            }
            0x13 => PlayPacket::PlayerRotation(PlayerRotation::proto_decode(src)?),
            0x19 => PlayPacket::PlayerAbilities(PlayerAbilities::proto_decode(src)?),
            0x1A => PlayPacket::PlayerDigging(PlayerDigging::proto_decode(src)?),
            0x1B => PlayPacket::EntityAction(EntityAction::proto_decode(src)?),
            0x23 => PlayPacket::HeldItemChange(HeldItemChange::proto_decode(src)?),
            0x26 => {
                PlayPacket::CreativeInventoryAction(CreativeInventoryAction::proto_decode(src)?)
            }
            0x2C => PlayPacket::PlayerBlockPlacement(PlayerBlockPlacement::proto_decode(src)?),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Unsupported PlayPacket: 0x{:>02x}", packet_id),
                ))
            }
        })
    }
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct TeleportConfirm {
    pub teleport_id: Var<i32>,
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
pub struct PlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerAbilities {
    pub flags: u8,
    pub flying_speed: f32,
    pub walking_speed: f32,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerDigging {
    pub status: DiggingStatus,
    pub location: Position,
    pub face: BlockFaceU8,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct EntityAction {
    pub entity_id: Var<i32>,
    pub action_id: Var<i32>,
    pub jump_boost: Var<i32>,
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct HeldItemChange {
    pub slot: i16,
}

#[derive(Debug, Clone)]
pub struct CreativeInventoryAction {
    pub slot: i16,
    pub clicked_item: Option<Slot>,
}

impl ProtocolRead for CreativeInventoryAction {
    fn proto_decode<S: Read>(src: &mut S) -> io::Result<Self> {
        let slot = i16::proto_decode(src)?;
        let present = bool::proto_decode(src)?;
        let clicked_item = if present {
            Some(Slot::proto_decode(src)?)
        } else {
            None
        };

        Ok(Self { slot, clicked_item })
    }
}

#[derive(Debug, Clone, ProtocolRead)]
pub struct PlayerBlockPlacement {
    pub hand: Var<i32>,
    pub location: Position,
    pub face: BlockFace,
    pub cursor_point_x: f32,
    pub cursor_point_y: f32,
    pub cursor_point_z: f32,
    pub inside_block: bool,
}
