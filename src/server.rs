use std::{
    io::{self, Cursor, ErrorKind, Result},
    thread::sleep,
    time::{Duration, Instant},
};

use aes::Aes128;
use cfb8::stream_cipher::NewStreamCipher;
use cfb8::{stream_cipher::StreamCipher, Cfb8};
use flume::Sender;
use futures_lite::FutureExt;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener, TcpStream,
    },
    time::timeout,
};

use crate::protocol::ProtocolRead;
use crate::{client::Client, packet::PacketWriteEnum, types::Var};
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
        for (index, client) in self.clients.iter_mut().enumerate() {
            match client.update_play() {
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
            println!("receive packet: {:?}", packet);
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

                        if let Ok(packet) = P::packet_read(&mut buf2_cur) {
                            return Ok(packet);
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
    play::chunk_data(worker).await?;
    play::world_border(worker).await?;
    play::spawn_position(worker).await?;
    play::play_position_and_look(worker).await?;

    Ok(())
}
