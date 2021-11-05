use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod mcstream;
mod packet;
mod protocol;
mod types;

mod login;
mod play;
mod slp;

use slp::{slp_ping, slp_status};

use packet::read_handshake_packet;
use packet::server::{HandshakePacket, NextState};

use aes::Aes128;
use cfb8::Cfb8;

pub type AesCfb8 = Cfb8<Aes128>;

use mcstream::McStream;

use crate::packet::{read_login_packet, read_status_packet};

fn handler(stream: TcpStream) -> Result<(), Error> {
    let mut stream = McStream::new(stream);
    let next_state = match read_handshake_packet(&mut stream)? {
        HandshakePacket::Handshake(handshake) => handshake.next_state,
    };
    match next_state {
        NextState::Status => loop {
            match read_status_packet(&mut stream)? {
                packet::server::StatusPacket::Request(request) => slp_status(&mut stream, request)?,
                packet::server::StatusPacket::Ping(ping) => slp_ping(&mut stream, ping)?,
            }
        },
        NextState::Login => {
            let rsa = openssl::rsa::Rsa::generate(1024).unwrap();

            let mut name = "".to_string();

            loop {
                match read_login_packet(&mut stream)? {
                    packet::server::LoginPacket::LoginStart(login_start) => {
                        let public_key = rsa.public_key_to_der()?;
                        let verify_token = vec![0u8, 123, 212, 123];
                        name = login::login_start(&mut stream, login_start)?;
                        println!("login attempt: {}", name);
                        login::encryption_request(&mut stream, public_key, verify_token)?;
                    }
                    packet::server::LoginPacket::EncryptionResponse(encryption_response) => {
                        let (name, id, props, key) = login::encryption_response(
                            &mut stream,
                            encryption_response,
                            &rsa,
                            &name,
                        )?;
                        println!(
                            "name: {}, id: {}, props: {:?}, key: {:?}",
                            name, id, props, key
                        );
                        stream.set_decryptor(&key);
                        stream.set_encryptor(&key);

                        login::set_compression(&mut stream)?;
                        login::login_success(&mut stream, id.to_string(), name)?;
                        break; // TODO: to play mode
                    }
                };
            }

            play::join_game(&mut stream)?;
            play::client_settings(&mut stream)?;
            play::held_item_change(&mut stream)?;
            play::declare_recipes(&mut stream)?;
            play::tags(&mut stream)?;
            play::entity_status(&mut stream)?;
            // play::decrale_commands(&mut stream)?;
            play::unlock_recipes(&mut stream)?;
            play::play_position_and_look(&mut stream)?;
            play::player_info(&mut stream)?;
            play::update_view_position(&mut stream)?;
            play::update_light(&mut stream)?;
            // play::chunk_data(&mut stream)?;
            play::world_border(&mut stream)?;
            play::spawn_position(&mut stream)?;
            play::play_position_and_look(&mut stream)?;
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
