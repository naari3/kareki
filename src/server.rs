use std::{
    io::{self, Cursor, ErrorKind, Read, Result},
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    thread::{self, sleep},
    time::{Duration, Instant},
};

use crate::protocol::ProtocolRead;
use crate::{
    login,
    mcstream::McStream,
    packet::{
        read_login_packet, read_play_packet,
        server::{LoginPacket, PlayPacket},
        PacketWrite,
    },
    play,
    state::State,
    types::Var,
    HandshakePacket,
};
use crate::{
    packet::{
        read_handshake_packet, read_status_packet,
        server::{NextState, StatusPacket},
    },
    slp::handle_slp_status,
};
use crate::{slp::handle_slp_ping, HandshakePacket::Handshake};

pub struct Client {
    stream: McStream,
    state: State,
    received_buf: Vec<u8>,
}

enum NextConnect {
    Disconnect,
    Join,
}

impl Client {
    fn handshake(&mut self, handshake: HandshakePacket) -> Result<NextConnect> {
        let next = match handshake {
            Handshake(h) => match h.next_state {
                NextState::Status => {
                    handle_status_handshake(&mut self.stream)?;
                    NextConnect::Disconnect
                }
                NextState::Login => {
                    handle_login_handshake(&mut self.stream, &mut self.state)?;
                    println!("gogo");

                    NextConnect::Join
                }
            },
        };
        Ok(next)
    }

    fn update_play(&mut self) -> Result<()> {
        match read_play_packet(&mut self.stream) {
            Ok(play_packet) => match play_packet {
                PlayPacket::ClientSettings(client_settings) => {
                    println!("client settings: {:?}", client_settings);
                }
                PlayPacket::KeepAlive(keep_alive) => {
                    println!("keep alive: {:?}", keep_alive);
                }
                PlayPacket::PlayerPositionAndRotation(player_position_and_rotation) => {
                    println!(
                        "player position and rotation: {:?}",
                        player_position_and_rotation
                    );
                }
            },
            Err(_) => {}
        }
        if self.state.last_keep_alive.elapsed().as_secs() > 10 {
            self.state.last_keep_alive = Instant::now();
            play::keep_alive(self)?;
        }
        Ok(())
    }

    pub fn write_packet<P>(&mut self, packet: P) -> Result<()>
    where
        P: PacketWrite,
    {
        packet.packet_write(&mut self.stream)?;
        Ok(())
    }

    pub fn read_packet_bytes<S>(&mut self, stream: &mut S) -> Result<Vec<u8>>
    where
        S: Read,
    {
        loop {
            if self.received_buf.len() > 0 {
                let mut cursor = Cursor::new(&self.received_buf[..]);
                let len: i32 = <Var<i32>>::proto_decode(&mut cursor)?.into();
                let mut data = Vec::with_capacity(len as usize);
                stream.read_exact(&mut data)?;
                return Ok(data);
            }

            let len = stream.read(&mut self.received_buf)?;
            if len == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "read 0 bytes").into());
            }
        }
    }
}

pub struct Server {
    clients: Vec<Client>,
    receiver: Receiver<Client>,
}

impl Server {
    pub fn listen(bind_address: &str, sender: mpsc::Sender<Client>) {
        let listener = TcpListener::bind(bind_address).expect("Error. failed to bind.");
        for streams in listener.incoming() {
            match streams {
                Err(e) => eprintln!("error: {}", e),
                Ok(tcp_stream) => {
                    println!("connection from {:?}", tcp_stream.peer_addr());
                    let mut stream = McStream::new(tcp_stream);
                    let state = State::default();

                    let handshake = read_handshake_packet(&mut stream).unwrap();
                    let mut client = Client {
                        stream,
                        state,
                        received_buf: Vec::with_capacity(1000000),
                    };
                    let next = match client.handshake(handshake) {
                        Ok(next) => next,
                        Err(err) => {
                            println!("{:?}", err);
                            continue;
                        }
                    };

                    match next {
                        NextConnect::Disconnect => continue,
                        NextConnect::Join => sender.send(client).unwrap(),
                    }
                }
            }
        }
    }

    pub fn new(bind_string: String) -> Self {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            Self::listen(&bind_string, sender);
        });
        Self {
            clients: Vec::new(),
            receiver,
        }
    }

    pub fn loop_once(&mut self) -> Result<()> {
        let start = Instant::now();
        self.update()?;
        let end = Instant::now();
        let duration = end - start;
        if duration < Duration::from_millis(50) {
            sleep(Duration::from_millis(50) - duration);
        }
        Ok(())
    }

    fn update(&mut self) -> Result<()> {
        loop {
            match self.receiver.try_recv() {
                Ok(stream) => self.clients.push(stream),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => return Ok(()),
            }
        }
        let mut will_remove = vec![];
        for (index, client) in self.clients.iter_mut().enumerate() {
            match client.update_play() {
                Ok(_) => {}
                Err(err) => {
                    match err.kind() {
                        ErrorKind::BrokenPipe => {
                            will_remove.push(index);
                        }
                        _ => {}
                    }
                    println!("err: {:?}", err);
                }
            }
        }
        for index in will_remove {
            self.clients.remove(index);
        }

        Ok(())
    }
}

pub fn handle_status_handshake(stream: &mut McStream) -> Result<()> {
    if let StatusPacket::Request(request) = read_status_packet(stream)? {
        handle_slp_status(stream, request)?;
    };
    if let StatusPacket::Ping(ping) = read_status_packet(stream)? {
        handle_slp_ping(stream, ping)?;
    };
    Ok(())
}

pub fn handle_login_handshake(stream: &mut McStream, state: &mut State) -> Result<()> {
    if let LoginPacket::LoginStart(start) = read_login_packet(stream)? {
        let crack = true;
        if crack {
            login::crack_login_start(stream, state, start)?;
        } else {
            login::login_start(stream, state, start)?;
        }
    }
    if !state.crack {
        if let LoginPacket::EncryptionResponse(encryption_response) = read_login_packet(stream)? {
            login::encryption_response(stream, state, encryption_response)?;
        }
    }

    play::join_game(stream)?;
    play::client_settings(stream)?;
    play::held_item_change(stream)?;
    play::declare_recipes(stream)?;
    play::tags(stream)?;
    play::entity_status(stream)?;
    // play::decrale_commands(&mut stream)?;
    play::unlock_recipes(stream)?;
    play::play_position_and_look(stream)?;
    play::player_info(stream, state)?;
    play::update_view_position(stream)?;
    play::update_light(stream)?;
    play::chunk_data(stream)?;
    play::world_border(stream)?;
    play::spawn_position(stream)?;
    play::play_position_and_look(stream)?;

    Ok(())
}
