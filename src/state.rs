use std::time::Instant;

use openssl::{pkey::Private, rsa::Rsa};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct State {
    pub name: Option<String>,
    pub rsa: Option<Rsa<Private>>,
    pub uuid: Option<Uuid>,
    pub crack: bool,
    pub coordinate: Coordinate,
    pub last_keep_alive: Instant,
}

impl Default for State {
    fn default() -> Self {
        Self {
            name: Default::default(),
            rsa: Default::default(),
            uuid: Default::default(),
            crack: false,
            coordinate: Default::default(),
            last_keep_alive: Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
