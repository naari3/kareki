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
            // use uuid::Uuid;

            let name = login::login_start(&mut stream)?;
            println!("login attempt: {}", name);

            let rsa = openssl::rsa::Rsa::generate(1024).unwrap();
            let pubkey = rsa.public_key_to_der()?;
            let verify_token = vec![0u8, 123, 212, 123];

            login::encryption_request(&mut stream, &pubkey, &verify_token)?;
            login::encryption_response(&mut stream, &rsa, &name)?;
            // login::login_success(&mut stream, &Uuid::new_v4(), &name)?;
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
