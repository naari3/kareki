use flume::{Receiver, Sender};
use std::{
    io::Result,
    time::{Instant, SystemTime},
};

use crate::{
    packet::{
        client::{self, BlockChange, KeepAlive},
        server::{
            self, CreativeInventoryAction, HeldItemChange, PlayPacket, PlayerBlockPlacement,
            PlayerPosition, PlayerPositionAndRotation, PlayerRotation,
        },
    },
    server::Server,
    state::{Coordinate, Rotation, State},
    types::slot::Slot,
};

pub struct Client {
    packets_to_send_tx: Sender<client::PlayPacket>,
    received_packets_rx: Receiver<server::PlayPacket>,
    pub state: State,
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

    pub fn received_packets(&self) -> Vec<PlayPacket> {
        self.received_packets_rx
            .try_iter()
            .collect::<Vec<_>>()
            .clone()
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

    pub fn set_position(&mut self, x: f64, y: f64, z: f64) -> Result<()> {
        self.state.coordinate = Coordinate { x, y, z };
        Ok(())
    }

    pub fn set_rotation(&mut self, yaw: f32, pitch: f32) -> Result<()> {
        self.state.rotation = Rotation { yaw, pitch };
        Ok(())
    }

    pub fn set_inventory_item(&mut self, slot_number: usize, item: Option<Slot>) -> Result<()> {
        self.state.inventory.slots[slot_number] = item;
        Ok(())
    }

    pub fn handle_block_placement(&mut self, placement: &PlayerBlockPlacement) -> Result<()> {
        let selected = if placement.hand.0 == 0 {
            self.state.inventory.selected + 36
        } else {
            45
        };
        let item = self.state.inventory.slots[selected].clone();

        if let Some(item) = item {
            let packet = client::PlayPacket::BlockChange(BlockChange {
                location: placement.location,
                block_id: item.item_id,
            });
            self.send_play_packet(packet)?;
        }

        Ok(())
    }
}
