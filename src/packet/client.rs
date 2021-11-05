use std::io::{self, Write};

use crate::{
    protocol::ProtocolWrite,
    types::{Arr, Var},
};

use super::PacketWrite;
pub enum _StatusPacket {
    SlpResponse(SlpResponse),
    Pong(Pong),
}

#[derive(Debug, Clone)]
pub struct SlpResponse {
    pub json_response: String,
}

impl PacketWrite for SlpResponse {}

impl ProtocolWrite for SlpResponse {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0, dst)?; // packet_id: 0

        String::proto_encode(&value.json_response, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Pong {
    pub payload: u64,
}

impl PacketWrite for Pong {}

impl ProtocolWrite for Pong {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&1, dst)?; // packet_id: 1

        u64::proto_encode(&value.payload, dst)?;
        Ok(())
    }
}

pub enum _Login {
    Disconnect(Disconnect),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
}

#[derive(Debug, Clone)]
pub struct Disconnect {
    pub chat: String,
}

impl PacketWrite for Disconnect {}

impl ProtocolWrite for Disconnect {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0, dst)?; // packet_id: 1

        String::proto_encode(&value.chat, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}
impl PacketWrite for EncryptionRequest {}

impl ProtocolWrite for EncryptionRequest {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&1, dst)?; // packet_id: 1

        String::proto_encode(&value.server_id, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.public_key, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.verify_token, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
impl PacketWrite for LoginSuccess {}

impl ProtocolWrite for LoginSuccess {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&2, dst)?; // packet_id: 2
        String::proto_encode(&value.uuid.to_string(), dst)?;
        String::proto_encode(&value.username, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SetCompression {
    pub thresshold: i32,
}
impl PacketWrite for SetCompression {}

impl ProtocolWrite for SetCompression {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&3, dst)?; // packet_id: 3
        <Var<i32>>::proto_encode(&value.thresshold, dst)?;
        Ok(())
    }
}
