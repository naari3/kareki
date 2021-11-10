mod mcstream;
pub mod packet;
mod protocol;
mod types;

mod login;
mod play;
mod slp;

mod server;
mod state;

pub use packet::server::{HandshakePacket, NextState};
use server::Server;

use aes::Aes128;
use cfb8::Cfb8;

pub type AesCfb8 = Cfb8<Aes128>;

#[tokio::main]
async fn main() {
    let mut server = Server::new("0.0.0.0:25565".to_string());
    loop {
        server.loop_once().unwrap();
    }
}
