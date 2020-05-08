use std::io::{self, Error, Read};

use super::types::string::decode_string;
use super::types::varint::decode_varint;

use byteorder::{BigEndian, ReadBytesExt};

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
            shared_secret_length: i32,
            shared_secret: Vec<u8>,
            verify_token_length: i32,
            verify_token: Vec<u8>,
        },
    }

    pub enum NextState {
        Status,
        Login,
    }
}

use serverbound::*;

fn read_packet_meta(stream: &mut dyn Read) -> Result<(u32, u32), Error> {
    let packet_size = decode_varint(stream)? as u32;
    let packet_id = decode_varint(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);
    Ok((packet_size, packet_id))
}

pub fn read_handshake_packet(stream: &mut dyn Read) -> Result<HandshakePacket, Error> {
    let (_, _) = read_packet_meta(stream)?;

    println!("get handshake");
    let protocol_version = decode_varint(stream)?;
    let server_address = decode_string(stream)?;
    let server_port = stream.read_u16::<BigEndian>()?;
    let next_state = match decode_varint(stream)? {
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
                payload: stream.read_u64::<BigEndian>()?,
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
                name: decode_string(stream)?,
            })
        }
        1 => {
            let shared_secret_length = decode_varint(stream)?;
            let mut shared_secret = vec![];
            for _ in 0..shared_secret_length {
                shared_secret.push(stream.read_u8()?);
            }

            let verify_token_length = decode_varint(stream)?;
            let mut verify_token = vec![];
            for _ in 0..verify_token_length {
                verify_token.push(stream.read_u8()?);
            }

            return Ok(LoginPacket::EncryptionResponse {
                shared_secret_length,
                shared_secret,
                verify_token_length,
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
