use std::io::{self, Error, Read, Seek, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod types;

use types::string::{decode_string, encode_string};
use types::varint::{decode_varint, encode_varint};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

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

            let json_response = r#"{"description":{"text":"A Minecraft Server"},"players":{"max":20,"online":12345},"version":{"name":"1.15.2","protocol":578}}"#;

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
