use std::io::{self, Error};
use std::net::{TcpListener, TcpStream};
use std::thread;

mod mcstream;
mod packet;
mod types;

mod login;
mod slp;

use slp::{slp_ping, slp_status};

use packet::{read_handshake_packet, HandshakePacket, NextState};

use aes::Aes128;
use cfb8::stream_cipher::NewStreamCipher;
use cfb8::Cfb8;

pub type AesCfb8 = Cfb8<Aes128>;

use mcstream::McStream;

fn handler(stream: TcpStream) -> Result<(), Error> {
    let mut stream = McStream::new(stream);
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

            let rsa = openssl::rsa::Rsa::generate(1024).unwrap();
            let pubkey = rsa.public_key_to_der()?;
            let verify_token = vec![0u8, 123, 212, 123];

            login::encryption_request(&mut stream, &pubkey, &verify_token)?;
            let (name, id, props, key) = login::encryption_response(&mut stream, &rsa, &name)?;
            println!(
                "name: {}, id: {}, props: {:?}, key: {:?}",
                name, id, props, key
            );
            stream.set_decryptor(&key);
            stream.set_encryptor(&key);

            login::set_compression(&mut stream)?;

            login::login_success(&mut stream, &id, &name)?;
        }
    };

    println!("==================================");
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:25565").expect("Error. failed to bind.");
    for streams in listener.incoming() {
        match streams {
            Err(e) => eprintln!("error: {}", e),
            Ok(stream) => {
                println!("connection from {:?}", stream.peer_addr());
                thread::spawn(move || {
                    handler(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
