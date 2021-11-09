use openssl::{pkey::Private, rsa::Rsa};
use uuid::Uuid;

pub struct State {
    pub mode: Mode,
    pub name: Option<String>,
    pub rsa: Option<Rsa<Private>>,
    pub uuid: Option<Uuid>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Handshake,
    Status,
    Login,
    Play,
    Finish,
}
