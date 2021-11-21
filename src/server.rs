use std::{
    io::{self, Cursor, ErrorKind, Result},
    thread::sleep,
    time::{Duration, Instant, SystemTime},
};

use aes::Aes128;
use cfb8::stream_cipher::NewStreamCipher;
use cfb8::{stream_cipher::StreamCipher, Cfb8};
use flume::Sender;
use futures_lite::FutureExt;
use kareki_data::{block::Block, item::Item};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};

use crate::{
    client::Client,
    packet::{
        client::{BlockChange, KeepAlive, UnloadChunk},
        server::{
            CreativeInventoryAction, HeldItemChange, PlayerBlockPlacement, PlayerDigging,
            PlayerPositionAndRotation, PlayerRotation,
        },
        PacketWriteEnum,
    },
    state::{Coordinate, Rotation},
    types::{digging_status::DiggingStatus, slot::Slot, Var},
    world::World,
};
use crate::{
    login,
    packet::{
        client,
        server::{LoginPacket, PlayPacket},
        PacketReadEnum,
    },
    play,
    state::State,
    HandshakePacket,
};
use crate::{packet::server::PlayerPosition, protocol::ProtocolRead};
use crate::{
    packet::server::{NextState, StatusPacket},
    slp::handle_slp_status,
};
use crate::{slp::handle_slp_ping, HandshakePacket::Handshake};

pub type AesCfb8 = Cfb8<Aes128>;

pub struct Worker {
    reader: Reader,
    writer: Writer,
    pub state: State,
    packets_to_send_tx: Sender<client::PlayPacket>,
    received_packets_rx: flume::Receiver<PlayPacket>,
}

pub enum NextConnect {
    Disconnect,
    Join,
}

impl Worker {
    fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        let (received_packets_tx, received_packets_rx) = flume::bounded(32);
        let (packets_to_send_tx, packets_to_send_rx) = flume::unbounded();
        let reader = Reader::new(reader, received_packets_tx);
        let writer = Writer::new(writer, packets_to_send_rx);

        Self {
            reader,
            writer,
            state: State::default(),
            packets_to_send_tx,
            received_packets_rx,
        }
    }

    fn run(self) {
        let Self { reader, writer, .. } = self;
        let reader = tokio::task::spawn(async move { reader.run().await });
        let writer = tokio::task::spawn(async move { writer.run().await });

        tokio::task::spawn(async move {
            let result = reader.race(writer).await.expect("task panicked");
            if let Err(_e) = result {
                // disconnect
            }
        });
    }

    async fn handshake(&mut self, handshake: HandshakePacket) -> Result<NextConnect> {
        let next = match handshake {
            Handshake(h) => match h.next_state {
                NextState::Status => {
                    handle_status_handshake(self).await?;
                    NextConnect::Disconnect
                }
                NextState::Login => {
                    handle_login_handshake(self).await?;
                    println!("gogo");

                    NextConnect::Join
                }
            },
        };
        Ok(next)
    }

    pub async fn write_packet<P: PacketWriteEnum>(&mut self, packet: P) -> Result<()> {
        self.writer.write(packet).await?;
        Ok(())
    }

    pub async fn read_packet_exact<P: PacketReadEnum>(&mut self) -> Result<P> {
        let packet = self.reader.read::<P>().await?;
        Ok(packet)
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.reader
            .set_decryptor(AesCfb8::new_var(&key, &key).unwrap());
        self.writer
            .set_encryptor(AesCfb8::new_var(&key, &key).unwrap());
    }

    pub fn packets_to_send(&self) -> Sender<client::PlayPacket> {
        self.packets_to_send_tx.clone()
    }

    pub fn received_packets(&self) -> flume::Receiver<PlayPacket> {
        self.received_packets_rx.clone()
    }
}

pub struct Server {
    clients: Vec<Client>,
    receiver: flume::Receiver<Client>,
    world: World,
}

impl Server {
    pub async fn listen(bind_address: &str, sender: Sender<Client>) {
        let mut listener = TcpListener::bind(bind_address)
            .await
            .expect("Error. failed to bind.");
        tokio::task::spawn(async move {
            loop {
                if let Ok((stream, addr)) = listener.accept().await {
                    println!("connection from {:?}", addr);

                    let mut worker = Worker::new(stream);
                    let handshake = worker.read_packet_exact::<HandshakePacket>().await.unwrap();
                    let next = match worker.handshake(handshake).await {
                        Ok(next) => next,
                        Err(err) => {
                            println!("{:?}", err);
                            continue;
                        }
                    };

                    match next {
                        NextConnect::Disconnect => continue,
                        NextConnect::Join => {
                            let state = worker.state.clone();
                            let client = Client::new(
                                worker.packets_to_send(),
                                worker.received_packets(),
                                state,
                            );
                            sender.send_async(client).await.unwrap();
                            worker.run()
                        }
                    }
                }
            }
        });
    }

    pub async fn new(bind_string: String) -> Self {
        let (sender, receiver) = flume::bounded(4);
        Self::listen(&bind_string, sender).await;

        Self {
            clients: Vec::new(),
            receiver,
            world: World::new().unwrap(),
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
                Ok(worker) => self.clients.push(worker),
                Err(flume::TryRecvError::Empty) => break,
                Err(flume::TryRecvError::Disconnected) => return Ok(()),
            }
        }
        let mut will_remove = vec![];
        let max_client_id = self.clients.len();
        for index in 0..max_client_id {
            match self.update_play(index) {
                Ok(_) => {}
                Err(err) => {
                    match err.kind() {
                        ErrorKind::BrokenPipe => {
                            will_remove.push(index);
                        }
                        _ => {
                            will_remove.push(index);
                        }
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

    pub fn update_play(&mut self, client_index: usize) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        if client.state.last_keep_alive.elapsed().as_secs() > 10 {
            client.state.last_keep_alive = Instant::now();
            client.keep_alive()?;
        }
        let packets = client.received_packets();
        for packet in packets.into_iter() {
            self.handle_packet(client_index, packet)?;
        }
        Ok(())
    }

    fn handle_packet(&mut self, client_index: usize, packet: PlayPacket) -> Result<()> {
        match packet {
            PlayPacket::ClientSettings(client_settings) => {
                println!("client settings: {:?}", client_settings);
            }
            PlayPacket::KeepAlive(_keep_alive) => {}
            PlayPacket::PlayerPosition(player_position) => {
                let PlayerPosition { x, feet_y, z, .. } = player_position;
                // println!("player_position: {:?}", player_position);
                self.set_position(client_index, x, feet_y, z)?;
            }
            PlayPacket::PlayerPositionAndRotation(player_position_and_rotation) => {
                let PlayerPositionAndRotation {
                    x,
                    feet_y,
                    z,
                    yaw,
                    pitch,
                    ..
                } = player_position_and_rotation;
                // println!(
                //     "player_position_and_rotation: {:?}",
                //     player_position_and_rotation
                // );
                self.set_position(client_index, x, feet_y, z)?;
                self.set_rotation(client_index, yaw, pitch)?;
            }
            PlayPacket::PlayerBlockPlacement(placement) => {
                self.handle_block_placement(client_index, &placement)?;
                // println!("item: {:?}", self.state.inventory.slots[placement.]);
            }
            PlayPacket::TeleportConfirm(teleport_confirm) => {
                println!("teleport_confirm: {:?}", teleport_confirm);

                let client = self.clients.get_mut(client_index).unwrap();

                let center_chunk_x = (client.state.coordinate.x as i32) >> 4;
                let center_chunk_z = (client.state.coordinate.x as i32) >> 4;

                let chunk = self.world.fetch_chunk(center_chunk_x, center_chunk_z)?;
                let packet = chunk.clone().to_packet(center_chunk_x, center_chunk_z)?;
                client.send_play_packet(packet)?;

                let view_distance = 2;
                let diff = view_distance * 1;
                for x in center_chunk_x - diff..=center_chunk_x + diff {
                    for z in center_chunk_z - diff..=center_chunk_z + diff {
                        if x == center_chunk_x && z == center_chunk_z {
                            continue;
                        }
                        let chunk = self.world.fetch_chunk(x, z)?;

                        let packet = chunk.clone().to_packet(x, z)?;

                        client.send_play_packet(packet)?;
                    }
                }
            }
            PlayPacket::PlayerRotation(player_rotation) => {
                let PlayerRotation { yaw, pitch, .. } = player_rotation;
                self.set_rotation(client_index, yaw, pitch)?;
            }
            PlayPacket::PlayerAbilities(player_abilities) => {
                println!("player_abilities: {:?}", player_abilities);
            }
            PlayPacket::PlayerDigging(player_digging) => {
                self.handle_block_digging(client_index, &player_digging)?;
            }
            PlayPacket::EntityAction(_entity_action) => {
                // println!("entity_action: {:?}", _entity_action);
            }
            PlayPacket::CreativeInventoryAction(creative_inventory_action) => {
                let CreativeInventoryAction {
                    slot: slot_number,
                    clicked_item,
                } = creative_inventory_action;
                self.set_inventory_item(client_index, slot_number as usize, clicked_item)?;
            }
            PlayPacket::HeldItemChange(held_item_change) => {
                let HeldItemChange { slot } = held_item_change;
                let client = self.clients.get_mut(client_index).unwrap();
                client.state.inventory.selected = slot as usize;
            }
        }

        Ok(())
    }

    pub fn keep_alive(&mut self, client_index: usize) -> Result<()> {
        let packet = client::PlayPacket::KeepAlive(KeepAlive {
            keep_alive_id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64,
        });

        let client = self.clients.get_mut(client_index).unwrap();
        client.send_play_packet(packet)?;

        Ok(())
    }

    pub fn set_position(&mut self, client_index: usize, x: f64, y: f64, z: f64) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        client.state.coordinate = Coordinate { x, y, z };

        let view_distance = 2;
        let chunk_x = x as i32 >> 4;
        let chunk_z = z as i32 >> 4;
        let last_chunk_x = client.state.last_chunk_x;
        let last_chunk_z = client.state.last_chunk_z;

        client.state.last_chunk_x = chunk_x;
        client.state.last_chunk_z = chunk_z;

        if last_chunk_x != chunk_x || last_chunk_z != chunk_z {
            let nx = last_chunk_x.min(chunk_x) - 2 * view_distance;
            let nz = last_chunk_z.min(chunk_z) - 2 * view_distance;
            let px = last_chunk_x.max(chunk_x) + 2 * view_distance;
            let pz = last_chunk_z.max(chunk_z) + 2 * view_distance;

            for x in nx..=px {
                for z in nz..=pz {
                    let was_loaded = Self::get_chunk_distance(x, z, last_chunk_x, last_chunk_z)
                        <= view_distance as u32;
                    let should_be_loaded =
                        Self::get_chunk_distance(x, z, chunk_x, chunk_z) <= view_distance as u32;

                    if !was_loaded && should_be_loaded {
                        // println!("load x: {:?} z: {:?}", x, z);
                        let chunk = self.world.fetch_chunk(x, z)?;

                        let packet = chunk.clone().to_packet(x, z)?;

                        client.send_play_packet(packet)?;
                    } else if was_loaded && !should_be_loaded {
                        // println!("unload x: {:?} z: {:?}", x, z);
                        let packet = client::PlayPacket::UnloadChunk(UnloadChunk {
                            chunk_x: x,
                            chunk_z: z,
                        });
                        client.send_play_packet(packet)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn set_rotation(&mut self, client_index: usize, yaw: f32, pitch: f32) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        client.state.rotation = Rotation { yaw, pitch };
        Ok(())
    }

    pub fn set_inventory_item(
        &mut self,
        client_index: usize,
        slot_number: usize,
        item: Option<Slot>,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        client.state.inventory.slots[slot_number] = item;
        Ok(())
    }

    pub fn handle_block_placement(
        &mut self,
        client_index: usize,
        placement: &PlayerBlockPlacement,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        let selected = if placement.hand.0 == 0 {
            client.state.inventory.selected + 36
        } else {
            45
        };
        let item = client.state.inventory.slots[selected].clone();

        if let Some(slot) = item {
            println!("placement: {:?}", placement);
            let block_pos = placement.location.offset(placement.face);
            let item = Item::from_id(slot.item_id.0 as _).expect("Unknown item id");
            println!("block_pos: {:?}, item: {:?}", block_pos, item);
            let block = Block::from_name(&item.name()).expect("Unknown block name");
            self.world.set_block(
                block_pos.x as usize,
                block_pos.y as usize,
                block_pos.z as usize,
                block.default_state() as u16,
            )?;

            let packet = client::PlayPacket::BlockChange(BlockChange {
                location: block_pos,
                block_id: Var(block.default_state() as i32),
            });
            client.send_play_packet(packet)?;
        }

        Ok(())
    }

    pub fn handle_block_digging(
        &mut self,
        client_index: usize,
        digging: &PlayerDigging,
    ) -> Result<()> {
        let client = self.clients.get_mut(client_index).unwrap();
        println!("digging: {:?}", digging);
        if let DiggingStatus::StartedDigging = digging.status {
            self.world.set_block(
                digging.location.x as usize,
                digging.location.y as usize,
                digging.location.z as usize,
                0,
            )?;
            let packet = client::PlayPacket::BlockChange(BlockChange {
                location: digging.location,
                block_id: Var(0),
            });
            client.send_play_packet(packet)?;
        }

        Ok(())
    }

    fn get_chunk_distance(x1: i32, z1: i32, x2: i32, z2: i32) -> u32 {
        let x = x1 - x2;
        let z = z1 - z2;
        x.abs().max(z.abs()) as u32
    }
}

pub struct Reader {
    stream: OwnedReadHalf,
    buffer: Vec<u8>,
    received_packets: Sender<PlayPacket>,
    decryptor: Option<AesCfb8>,
}

impl Reader {
    pub fn new(stream: OwnedReadHalf, received_packets: Sender<PlayPacket>) -> Self {
        Self {
            stream,
            buffer: vec![],
            received_packets,
            decryptor: None,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        loop {
            let packet = self.read::<PlayPacket>().await?;
            let result = self.received_packets.send_async(packet).await;
            if result.is_err() {
                return Ok(());
            }
        }
    }

    pub async fn read<P: PacketReadEnum>(&mut self) -> Result<P> {
        loop {
            let mut cursor = Cursor::new(&self.buffer[..]);
            match <Var<i32>>::proto_decode(&mut cursor) {
                Ok(length) => {
                    let length: i32 = length.into();
                    let length = length as usize;
                    if self.buffer.len() >= length {
                        let mut buf2 = vec![0; length];
                        std::io::Read::read_exact(&mut cursor, &mut buf2[..])?;
                        let mut buf2_cur = Cursor::new(buf2);

                        let next_cursor = cursor.position() as usize;
                        self.buffer = self.buffer.split_off(next_cursor);

                        match P::packet_read(&mut buf2_cur) {
                            Ok(packet) => return Ok(packet),
                            Err(err) => println!("err: {:?}", err),
                        }
                    }
                }
                Err(_) => {}
            }

            let duration = Duration::from_secs(10);
            let mut buf = vec![0; 512];
            let read_bytes = timeout(duration, self.stream.read(&mut buf)).await??;
            if read_bytes == 0 {
                return Err(io::Error::new(ErrorKind::UnexpectedEof, "read 0 bytes").into());
            }

            let bytes = &mut buf[..read_bytes];
            if let Some(decryptor) = self.decryptor.as_mut() {
                decryptor.decrypt(bytes);
            }
            self.buffer.extend(bytes.as_ref());
        }
    }

    pub fn set_decryptor(&mut self, decryptor: AesCfb8) {
        self.decryptor = Some(decryptor);
    }
}

pub struct Writer {
    stream: OwnedWriteHalf,
    packets_to_send: flume::Receiver<client::PlayPacket>,
    encryptor: Option<AesCfb8>,
}

impl Writer {
    pub fn new(
        stream: OwnedWriteHalf,
        packets_to_send: flume::Receiver<client::PlayPacket>,
    ) -> Self {
        Self {
            stream,
            packets_to_send,
            encryptor: None,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        while let Ok(packet) = self.packets_to_send.recv_async().await {
            self.write(packet).await?;
        }
        Ok(())
    }

    pub async fn write<P: PacketWriteEnum>(&mut self, packet: P) -> Result<()> {
        let mut dst = Vec::new();
        packet.packet_write(&mut dst)?;
        if let Some(encryptor) = self.encryptor.as_mut() {
            encryptor.encrypt(&mut dst);
        }
        self.stream.write_all(&dst).await?;
        Ok(())
    }

    pub fn set_encryptor(&mut self, encryptor: AesCfb8) {
        self.encryptor = Some(encryptor);
    }
}

pub async fn handle_status_handshake(worker: &mut Worker) -> Result<()> {
    if let StatusPacket::Request(request) = worker.read_packet_exact::<StatusPacket>().await? {
        handle_slp_status(worker, request).await?;
    };
    if let StatusPacket::Ping(ping) = worker.read_packet_exact::<StatusPacket>().await? {
        handle_slp_ping(worker, ping).await?;
    };
    Ok(())
}

pub async fn handle_login_handshake(worker: &mut Worker) -> Result<()> {
    if let LoginPacket::LoginStart(start) = worker.read_packet_exact().await? {
        let crack = false;
        if crack {
            login::crack_login_start(worker, start).await?;
        } else {
            login::login_start(worker, start).await?;
        }
    }
    if !worker.state.crack {
        if let LoginPacket::EncryptionResponse(encryption_response) =
            worker.read_packet_exact().await?
        {
            login::encryption_response(worker, encryption_response).await?;
        }
    }

    play::join_game(worker).await?;
    play::client_settings(worker).await?;
    play::held_item_change(worker).await?;
    play::declare_recipes(worker).await?;
    play::tags(worker).await?;
    play::entity_status(worker).await?;
    // play::decrale_commands(&mut stream)?;
    play::unlock_recipes(worker).await?;
    play::play_position_and_look(worker).await?;
    play::player_info(worker).await?;
    play::update_view_position(worker).await?;
    play::update_light(worker).await?;
    // play::chunk_data(worker).await?;
    play::world_border(worker).await?;
    play::spawn_position(worker).await?;
    play::play_position_and_look(worker).await?;

    Ok(())
}
