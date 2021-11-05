use std::io::{self, Error};

use super::mcstream::McStream;

use crate::packet::client::{Disconnect, EncryptionRequest, LoginSuccess, SetCompression};
use crate::packet::server::{EncryptionResponse, LoginStart};
use crate::packet::PacketWrite;

use super::packet::{read_login_packet, server::LoginPacket};

use uuid::Uuid;

use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};

pub fn disconnect(stream: &mut McStream) -> Result<(), Error> {
    let disconnect = Disconnect {
        chat: r#"{"text": "konaide ( ; _ ; )"}"#.to_string(),
    };
    disconnect.packet_write(stream)?;

    println!("disconnected");
    Ok(())
}

pub fn login_start(stream: &mut McStream, login_start: LoginStart) -> Result<String, Error> {
    return Ok(login_start.name);
}

pub fn encryption_request(
    stream: &mut McStream,
    public_key: Vec<u8>,
    verify_token: Vec<u8>,
) -> Result<(), Error> {
    let encryption_request = EncryptionRequest {
        server_id: "".to_string(),
        public_key,
        verify_token,
    };
    encryption_request.packet_write(stream)?;

    println!("sent encrypted request");
    Ok(())
}

pub fn encryption_response(
    stream: &mut McStream,
    enctyption_response: EncryptionResponse,
    rsa: &Rsa<Private>,
    name: &String,
) -> Result<(String, Uuid, Vec<mojang_api::ProfileProperty>, [u8; 16]), Error> {
    println!("receive encryption response");
    // use mojang_api::ServerAuthResponse;

    let mut decoded_shared_secret = vec![0; rsa.size() as usize];
    rsa.private_decrypt(
        &enctyption_response.shared_secret,
        &mut decoded_shared_secret,
        Padding::PKCS1,
    )?;
    let mut decoded_verify_token = vec![0; rsa.size() as usize];
    rsa.private_decrypt(
        &enctyption_response.verify_token,
        &mut decoded_verify_token,
        Padding::PKCS1,
    )?;
    println!("shared_secret: {:?}", decoded_shared_secret);
    println!("verify_token:  {:?}", decoded_verify_token);

    let mut key = [0u8; 16];
    for (i, x) in decoded_shared_secret[..16].iter().enumerate() {
        key[i] = *x;
    }

    let server_hash = mojang_api::server_hash("", key, &rsa.public_key_to_der()?);
    let auth_result = mojang_api::server_auth(&server_hash, name);

    let mut rt = tokio::runtime::Runtime::new()?;
    match rt.block_on(auth_result) {
        Ok(auth) => {
            return Ok((auth.name, auth.id, auth.properties, key));
        }
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("auth failed {:?}", e),
            ))
        }
    };
}

pub fn set_compression(stream: &mut McStream) -> Result<(), Error> {
    let set_compression = SetCompression {
        thresshold: -1, // this mean do not compression
    };
    set_compression.packet_write(stream)?;

    Ok(())
}

pub fn login_success(stream: &mut McStream, uuid: String, username: String) -> Result<(), Error> {
    let login_success = LoginSuccess { uuid, username };
    login_success.packet_write(stream)?;

    println!("login successful");
    Ok(())
}
