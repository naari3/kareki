use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod types;

use types::string::{decode_string};
use types::varint::{decode_varint};

use byteorder::{BigEndian, ReadBytesExt};

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);
    let mut state = 0; 
    loop {
        let packet_size = decode_varint(&mut stream)? as u32;
        let packet_id = decode_varint(&mut stream)? as u32;
        println!("packet size: {}, packet_id: {}", packet_size, packet_id);

        if state == 0 && packet_id == 0 {
            let protocol_version = decode_varint(&mut stream)?;
            let server_address = decode_string(&mut stream)?;
            let server_port = stream.read_u16::<BigEndian>()?;
            let next_state = decode_varint(&mut stream)?;

            println!(
                "protocol_version: {}, server_address: {}, server_port: {}, next_state: {}",
                protocol_version, server_address, server_port, next_state
            );

            state = next_state;
        }


        let mut buffer = [0u8; 1024];
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            return Ok(());
        }
        println!(
            "Message from {}: {:?}",
            stream.peer_addr()?,
            &buffer[..nbytes]
        );
        stream.write(&buffer[..nbytes])?;
        stream.flush()?;
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