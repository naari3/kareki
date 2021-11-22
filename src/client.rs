use flume::{Receiver, Sender};
use std::{cell::Cell, io::Result, time::SystemTime};

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
    pub state: State,
    pub is_disconnected: Cell<bool>,
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
            is_disconnected: Cell::new(false),
        }
    }

    pub fn received_packets(&self) -> Vec<PlayPacket> {
        self.received_packets_rx
            .try_iter()
            .collect::<Vec<_>>()
            .clone()
    }

    pub fn is_disconnected(&self) -> bool {
        self.received_packets_rx.is_disconnected() || self.is_disconnected.get()
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
