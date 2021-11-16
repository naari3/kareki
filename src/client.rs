use flume::{Receiver, Sender};
use std::{
    io::Result,
    time::{Instant, SystemTime},
};

use crate::{
    packet::{
        client::{self, KeepAlive},
        server::{self, PlayPacket, PlayerPosition, PlayerPositionAndRotation},
    },
    state::{Coordinate, State},
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
        for packet in packets.into_iter() {
            if self.state.last_keep_alive.elapsed().as_secs() > 10 {
                self.state.last_keep_alive = Instant::now();
                self.keep_alive()?;
            }
            self.handle_packet(packet)?;
        }
        Ok(())
    }

    pub fn send_play_packet(&self, packet: client::PlayPacket) -> Result<()> {
        let _ = self.packets_to_send_tx.try_send(packet);
        Ok(())
    }
}

impl Client {
    fn handle_packet(&mut self, packet: PlayPacket) -> Result<()> {
        match packet {
            PlayPacket::ClientSettings(client_settings) => {
                println!("client settings: {:?}", client_settings);
            }
            PlayPacket::KeepAlive(_keep_alive) => {}
            PlayPacket::PlayerPosition(player_position) => {
                let PlayerPosition { x, feet_y, z, .. } = player_position;
                self.set_position(x, feet_y, z)?;
            }
            PlayPacket::PlayerPositionAndRotation(player_position_and_rotation) => {
                let PlayerPositionAndRotation { x, feet_y, z, .. } = player_position_and_rotation;
                self.set_position(x, feet_y, z)?;
            }
            PlayPacket::PlayerBlockPlacement(placement) => {
                println!("placement: {:?}", placement);
            }
        }

        Ok(())
    }

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

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) -> Result<()> {
        self.state.coordinate = Coordinate { x, y, z };
        Ok(())
    }
}
