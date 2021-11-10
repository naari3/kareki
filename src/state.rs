use openssl::{pkey::Private, rsa::Rsa};
use uuid::Uuid;

pub struct State {
    pub mode: Mode,
    pub name: Option<String>,
    pub rsa: Option<Rsa<Private>>,
    pub uuid: Option<Uuid>,
    pub crack: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: Mode::Handshake,
            name: Default::default(),
            rsa: Default::default(),
            uuid: Default::default(),
            crack: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Handshake,
    Status,
    Login,
    Play,
}
