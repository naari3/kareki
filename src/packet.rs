use std::io::{self, Error};
use std::net::TcpStream;

use super::types::string::decode_string;
use super::types::varint::decode_varint;

use byteorder::{BigEndian, ReadBytesExt};


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
    Ping {
        payload: u64,
    },
}

pub enum NextState {
    Status,
    Login,
}

fn read_packet_meta(stream: &mut TcpStream) -> Result<(u32, u32), Error> {
    let packet_size = decode_varint(stream)? as u32;
    let packet_id = decode_varint(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);
    Ok((packet_size, packet_id))
}

pub fn read_handshake_packet(stream: &mut TcpStream) -> Result<HandshakePacket, Error> {
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

pub fn read_status_packet(stream: &mut TcpStream) -> Result<StatusPacket, Error> {
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
