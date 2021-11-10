use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod mcstream;
pub mod packet;
mod protocol;
mod types;

mod login;
mod play;
mod slp;

mod server;
mod state;

use server::Server;
use slp::{handle_slp_ping, handle_slp_status};

use packet::read_handshake_packet;
pub use packet::server::{HandshakePacket, NextState};

use aes::Aes128;
use cfb8::Cfb8;

pub type AesCfb8 = Cfb8<Aes128>;

use mcstream::McStream;

use crate::packet::server::{LoginPacket, StatusPacket};
use crate::packet::{read_login_packet, read_status_packet};
use crate::state::Mode;

fn handler(stream: TcpStream) -> Result<(), Error> {
    let mut stream = McStream::new(stream);
    let mut state = state::State::default();
    loop {
        match &state.mode {
            Mode::Handshake => {
                match read_handshake_packet(&mut stream)? {
                    HandshakePacket::Handshake(handshake) => match handshake.next_state {
                        NextState::Status => state.mode = Mode::Status,
                        NextState::Login => {
                            state.mode = Mode::Login;
                            state.rsa = Some(openssl::rsa::Rsa::generate(1024).unwrap());
                        }
                    },
                };
            }
            Mode::Status => {
                match read_status_packet(&mut stream)? {
                    StatusPacket::Request(request) => handle_slp_status(&mut stream, request)?,
                    StatusPacket::Ping(ping) => handle_slp_ping(&mut stream, ping)?,
                };
            }
            Mode::Login => {
                match read_login_packet(&mut stream)? {
                    LoginPacket::LoginStart(login_start) => {
                        let crack = true;
                        if crack {
                            login::crack_login_start(&mut stream, &mut state, login_start)?;
                            state.mode = Mode::Play;
                            continue;
                        } else {
                            login::login_start(&mut stream, &mut state, login_start)?
                        }
                    }
                    LoginPacket::EncryptionResponse(encryption_response) => {
                        login::encryption_response(&mut stream, &mut state, encryption_response)?
                    }
                };
            }
            Mode::Play => {
                play::join_game(&mut stream)?;
                play::client_settings(&mut stream)?;
                play::held_item_change(&mut stream)?;
                play::declare_recipes(&mut stream)?;
                play::tags(&mut stream)?;
                play::entity_status(&mut stream)?;
                // play::decrale_commands(&mut stream)?;
                play::unlock_recipes(&mut stream)?;
                play::play_position_and_look(&mut stream)?;
                play::player_info(&mut stream, &mut state)?;
                play::update_view_position(&mut stream)?;
                play::update_light(&mut stream)?;
                play::chunk_data(&mut stream)?;
                play::world_border(&mut stream)?;
                play::spawn_position(&mut stream)?;
                play::play_position_and_look(&mut stream)?;

                break;
            }
        }
    }

    println!("==================================");
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut server = Server::new("0.0.0.0:25565".to_string());
    loop {
        server.loop_once().unwrap();
    }
}
