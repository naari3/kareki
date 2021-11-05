use std::io::{self, Error, Write};

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use byteorder::{BigEndian, WriteBytesExt};

use crate::packet::client::{Pong, SlpResponse};
use crate::packet::PacketWrite;
use crate::protocol::ProtocolWrite;
use crate::types::Var;

use super::mcstream::McStream;

use super::packet::{read_status_packet, server::StatusPacket};

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

pub fn slp_status(stream: &mut McStream) -> Result<(), Error> {
    match read_status_packet(stream)? {
        StatusPacket::Request => {
            println!("get status request");

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

            let slp_response = SlpResponse { json_response };
            slp_response.packet_write(stream)?;

            println!("sent status");
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

pub fn slp_ping(stream: &mut McStream) -> Result<(), Error> {
    match read_status_packet(stream)? {
        StatusPacket::Ping { payload } => {
            println!("get ping");

            let pong = Pong { payload };
            pong.packet_write(stream)?;

            println!("sent pong");
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
