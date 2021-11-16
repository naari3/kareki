use flume::{Receiver, Sender};
use std::{
    io::Result,
    time::{Instant, SystemTime},
};

use crate::{
    packet::{
        client::{self, KeepAlive},
        server::{self, PlayPacket},
    },
    state::State,
};

pub struct Client {
    packets_to_send_tx: Sender<client::PlayPacket>,
    received_packets_rx: Receiver<server::PlayPacket>,
    state: State,
}

impl Client {
    pub fn new(
        packets_to_send_tx: Sender<client::PlayPacket>,
        received_packets_rx: Receiver<server::PlayPacket>,
        state: State,
    ) -> Self {
        Self {
            packets_to_send_tx,
            received_packets_rx,
            state,
        }
    }

    pub fn update_play(&mut self) -> Result<()> {
        let packets = self.received_packets_rx.try_iter().collect::<Vec<_>>();
        for packet in packets.iter() {
            if self.state.last_keep_alive.elapsed().as_secs() > 10 {
                self.state.last_keep_alive = Instant::now();
                self.keep_alive()?;
            }
            match packet {
                PlayPacket::ClientSettings(client_settings) => {
                    println!("client settings: {:?}", client_settings);
                }
                PlayPacket::KeepAlive(keep_alive) => {
                    println!("keep alive: {:?}", keep_alive);
                }
                PlayPacket::PlayerPosition(player_position) => {
                    println!("player position: {:?}", player_position);
                }
                PlayPacket::PlayerPositionAndRotation(player_position_and_rotation) => {
                    println!(
                        "player position and rotation: {:?}",
                        player_position_and_rotation
                    );
                }
                PlayPacket::PlayerBlockPlacement(placement) => {
                    println!("placement: {:?}", placement);
                }
            }
        }
        Ok(())
    }

    pub fn send_play_packet(&self, packet: client::PlayPacket) -> Result<()> {
        let _ = self.packets_to_send_tx.try_send(packet);
        Ok(())
    }
}

impl Client {
    pub fn keep_alive(&self) -> Result<()> {
        let packet = client::PlayPacket::KeepAlive(KeepAlive {
            keep_alive_id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        });

        self.send_play_packet(packet)?;

        Ok(())
    }
}
