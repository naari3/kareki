use std::io::{self, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod packet;
mod types;

mod slp;

mod login;

use slp::{slp_ping, slp_status};

use packet::{read_handshake_packet, HandshakePacket, NextState};

fn handler(mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from {}", stream.peer_addr()?);

    let next_state = match read_handshake_packet(&mut stream)? {
        HandshakePacket::Handshake { next_state, .. } => next_state,
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
        NextState::Login => {
            let name = login::login_start(&mut stream)?;
            println!("login attempt: {}", name);
            login::disconnect(&mut stream)?;
        }
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
