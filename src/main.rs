use std::io::{self, Error, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod types;

use types::string::{decode_string, encode_string};
use types::varint::{decode_varint, encode_varint};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Description {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Option<Vec<Sample>>,
}

#[derive(Debug, Serialize)]
pub struct Sample {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug)]
pub struct StatusResponse {
    pub description: Description,
    pub favicon: Option<String>,
    pub players: Players,
    pub version: Version,
}

impl Serialize for StatusResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("StatusResponse", 3)?;
        s.serialize_field("description", &self.description)?;
        if self.favicon.is_some() {
            s.serialize_field("favicon", &self.favicon)?;
        }
        s.serialize_field("players", &self.players)?;
        s.serialize_field("version", &self.version)?;
        s.end()
    }
}

enum Packet {
    Handshake {
        protocol_version: i32,
        server_address: String,
        server_port: u16,
        next_state: NextState,
    },
    Request,
    Ping {
        payload: u64,
    },
}

enum NextState {
    Status,
    Login,
}

fn read_handshake_packet(stream: &mut TcpStream) -> Result<Packet, Error> {
    let packet_size = decode_varint(stream)? as u32;
    let packet_id = decode_varint(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);

    println!("get handshake");
    let protocol_version = decode_varint(stream)?;
    let server_address = decode_string(stream)?;
    let server_port = stream.read_u16::<BigEndian>()?;
    let next_state = match decode_varint(stream)? {
        1 => NextState::Status,
        2 => NextState::Login,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid state")),
    };

    Ok(Packet::Handshake {
        protocol_version: protocol_version,
        server_address: server_address,
        server_port: server_port,
        next_state: next_state,
    })
}

fn read_status_packet(stream: &mut TcpStream) -> Result<Packet, Error> {
    let packet_size = decode_varint(stream)? as u32;
    let packet_id = decode_varint(stream)? as u32;
    println!("packet size: {}, packet_id: {}", packet_size, packet_id);

    match packet_id {
        0 => return Ok(Packet::Request),
        1 => {
            return Ok(Packet::Ping {
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

fn slp_status(stream: &mut TcpStream) -> Result<(), Error> {
    match read_status_packet(stream)? {
        Packet::Request => {
            println!("get status request");
            let mut r = io::Cursor::new(vec![] as Vec<u8>);

            let status_response = StatusResponse {
                description: Description {
                    text: "Yo this was implemented by naari3 @naari_ @_naari_".to_string(),
                },
                players: Players {
                    max: 12345,
                    online: 126534640, // Japan population
                    sample: Some(vec![]),
                },
                favicon: None,
                version: Version {
                    name: "1.15.2".to_string(),
                    protocol: 578,
                },
            };

            let json_response = serde_json::to_string(&status_response)?;

            println!("will send: {}", json_response);

            encode_varint(&0, &mut r)?; // packet_id: 0
            encode_string(&json_response.to_string(), &mut r)?;

            println!("packet size: {}", r.get_ref().len() as i32);

            encode_varint(&(r.get_ref().len() as i32), stream)?;
            stream.write_all(r.get_ref())?;

            println!("sent status");
            stream.flush()?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }
    Ok(())
}

fn slp_ping(stream: &mut TcpStream) -> Result<(), Error> {
    match read_status_packet(stream)? {
        Packet::Ping { payload } => {
            println!("get ping");

            let mut r = io::Cursor::new(vec![] as Vec<u8>);

            encode_varint(&1, &mut r)?;
            r.write_u64::<BigEndian>(payload)?;

            encode_varint(&(r.get_ref().len() as i32), stream)?;
            stream.write_all(&r.get_ref())?;

            println!("sent pong");
            stream.flush()?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }
    Ok(())
}

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);

    let next_state = match read_handshake_packet(&mut stream)? {
        Packet::Handshake { next_state, .. } => next_state,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet",
            ))
        }
    };
    match next_state {
        NextState::Status => {
            slp_status(&mut stream)?;
            slp_ping(&mut stream)?;
        }
        NextState::Login => {}
    };

    Ok(())
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => eprintln!("error: {}", e),
            Ok(stream) => {
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
