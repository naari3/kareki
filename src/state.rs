use openssl::{pkey::Private, rsa::Rsa};

pub struct State {
    pub mode: Mode,
    pub name: Option<String>,
    pub rsa: Option<Rsa<Private>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Handshake,
    Status,
    Login,
    Play,
    Finish,
}
