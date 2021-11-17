use std::time::Instant;

use openssl::{pkey::Private, rsa::Rsa};
use uuid::Uuid;

use crate::types::slot::Slot;

#[derive(Debug, Clone)]
pub struct State {
    pub name: Option<String>,
    pub rsa: Option<Rsa<Private>>,
    pub uuid: Option<Uuid>,
    pub crack: bool,
    pub coordinate: Coordinate,
    pub rotation: Rotation,
    pub inventory: Inventory,
    pub last_keep_alive: Instant,
    pub last_chunk_x: i32,
    pub last_chunk_z: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: Default::default(),
            rsa: Default::default(),
            uuid: Default::default(),
            crack: false,
            coordinate: Default::default(),
            rotation: Default::default(),
            inventory: Default::default(),
            last_keep_alive: Instant::now(),
            last_chunk_x: 0,
            last_chunk_z: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub slots: Vec<Option<Slot>>,
    pub selected: usize,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![Default::default(); 46],
            selected: 0,
        }
    }
}
