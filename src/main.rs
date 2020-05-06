use std::io::{self, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod packet;
mod slp;
mod types;

use slp::{slp_ping, slp_status};

use packet::{read_handshake_packet, NextState, Packet};

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

    println!("==================================");
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
