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

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut state = 0;
    loop {
        println!("state: {}", state);

        let packet_size = decode_varint(&mut stream)? as u32;
        let packet_id = decode_varint(&mut stream)? as u32;
        println!("packet size: {}, packet_id: {}", packet_size, packet_id);

        if state == 0 && packet_id == 0 {
            println!("get handshake");
            let protocol_version = decode_varint(&mut stream)?;
            let server_address = decode_string(&mut stream)?;
            let server_port = stream.read_u16::<BigEndian>()?;
            let next_state = decode_varint(&mut stream)?;

            println!(
                "protocol_version: {}, server_address: {}, server_port: {}, next_state: {}",
                protocol_version, server_address, server_port, next_state
            );

            state = next_state;
            stream.flush()?;
        } else if state == 1 && packet_id == 0 {
            println!("get status request");
            let mut r = io::Cursor::new(vec![] as Vec<u8>);
            let mut dst = io::Cursor::new(vec![] as Vec<u8>);

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

            encode_varint(&(r.get_ref().len() as i32), &mut dst)?;
            dst.write_all(r.get_ref())?;

            stream.write_all(&dst.get_ref())?;
            println!("sent status");
            stream.flush()?;
        } else if state == 1 && packet_id == 1 {
            println!("get ping");
            let payload = stream.read_u64::<BigEndian>()?;

            let mut r = io::Cursor::new(vec![] as Vec<u8>);
            let mut dst = io::Cursor::new(vec![] as Vec<u8>);

            encode_varint(&1, &mut r)?;
            r.write_u64::<BigEndian>(payload)?;

            encode_varint(&(r.get_ref().len() as i32), &mut dst)?;
            dst.write_all(&r.get_ref())?;

            stream.write_all(&dst.get_ref())?;
            println!("sent pong");
            stream.flush()?;
        }
    }
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
