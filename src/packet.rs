use std::io::{self, Error, Read};

use crate::protocol::Protocol;
use crate::types::{Arr, Var};

pub mod clientbound {
    pub enum StatusPacket {
        Response {
            json_length: i32,
            json_response: String,
        },
        Pong {
            payload: u64,
        },
    }
    pub enum Login {
        Disconnect {
            chat: String,
        },
        EncryptionRequest {
            server_id: String,
            public_key: Vec<u8>,
            verify_token: Vec<u8>,
        },
        LoginSuccess {
            uuid: String,
            username: String,
        },
    }
}

pub mod serverbound {
    pub enum HandshakePacket {
        Handshake {
            protocol_version: i32,
            server_address: String,
            server_port: u16,
            next_state: NextState,
        },
    }

    pub enum StatusPacket {
        Request,
        Ping { payload: u64 },
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

    pub enum NextState {
        Status,
        Login,
    }
}

use serverbound::*;

fn read_packet_meta(stream: &mut dyn Read) -> Result<(u32, u32), Error> {
    let packet_size = <Var<i32>>::proto_decode(stream)? as u32;
    let packet_id = <Var<i32>>::proto_decode(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);
    Ok((packet_size, packet_id))
}

pub fn read_handshake_packet(stream: &mut dyn Read) -> Result<HandshakePacket, Error> {
    let (_, _) = read_packet_meta(stream)?;

    println!("get handshake");
    let protocol_version = <Var<i32>>::proto_decode(stream)?;
    let server_address = String::proto_decode(stream)?;
    let server_port = u16::proto_decode(stream)?;
    let next_state = match <Var<i32>>::proto_decode(stream)? {
        1 => NextState::Status,
        2 => NextState::Login,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")),
    };

    Ok(HandshakePacket::Handshake {
        protocol_version: protocol_version,
        server_address: server_address,
        server_port: server_port,
        next_state: next_state,
    })
}

pub fn read_status_packet(stream: &mut dyn Read) -> Result<StatusPacket, Error> {
    let (_, packet_id) = read_packet_meta(stream)?;

    match packet_id {
        0 => return Ok(StatusPacket::Request),
        1 => {
            return Ok(StatusPacket::Ping {
                payload: u64::proto_decode(stream)?,
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
