use std::io::{self, Error, Write};
use std::net::TcpStream;

use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

use byteorder::{BigEndian, WriteBytesExt};

use super::types::string::encode_string;
use super::types::varint::encode_varint;

use super::packet::{Packet, read_status_packet};

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

pub fn slp_status(stream: &mut TcpStream) -> Result<(), Error> {
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

pub fn slp_ping(stream: &mut TcpStream) -> Result<(), Error> {
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
