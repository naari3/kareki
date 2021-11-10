use std::io::{self, Error};
use std::str::FromStr;

use super::mcstream::McStream;

use crate::packet::client::{Disconnect, EncryptionRequest, LoginSuccess, SetCompression};
use crate::packet::server::{EncryptionResponse, LoginStart};
use crate::packet::PacketWrite;
use crate::state::{Mode, State};

use openssl::rsa::Padding;
use uuid::Uuid;

pub fn crack_login_start(
    stream: &mut McStream,
    state: &mut State,
    _login_start: LoginStart,
) -> Result<(), Error> {
    // crack
    let uuid = Uuid::from_str("af6c5ee2-8eeb-8099-b8ce-253c50b0d8a8").unwrap();
    login_success(stream, uuid.to_string(), "todo".to_string())?;
    state.uuid = Some(uuid);
    state.name = Some("todo".to_string());
    state.crack = true;

    return Ok(());
}

pub fn login_start(
    stream: &mut McStream,
    state: &mut State,
    login_start: LoginStart,
) -> Result<(), Error> {
    state.rsa = Some(openssl::rsa::Rsa::generate(1024).unwrap());
    let public_key = state
        .rsa
        .as_ref()
        .expect("maybe not in login mode")
        .public_key_to_der()?;
    let verify_token = vec![0u8, 123, 212, 123];
    println!("login attempt: {}", login_start.name);
    state.name = Some(login_start.name);
    encryption_request(stream, public_key, verify_token)?;

    return Ok(());
}

pub fn encryption_response(
    stream: &mut McStream,
    state: &mut State,
    encryption_response: EncryptionResponse,
) -> Result<(), Error> {
    println!("receive encryption response");
    // use mojang_api::ServerAuthResponse;

    let mut decoded_shared_secret = vec![0; state.rsa.as_ref().unwrap().size() as usize];
    state.rsa.as_ref().unwrap().private_decrypt(
        &encryption_response.shared_secret,
        &mut decoded_shared_secret,
        Padding::PKCS1,
    )?;
    let mut decoded_verify_token = vec![0; state.rsa.as_ref().unwrap().size() as usize];
    state.rsa.as_ref().unwrap().private_decrypt(
        &encryption_response.verify_token,
        &mut decoded_verify_token,
        Padding::PKCS1,
    )?;
    println!("shared_secret: {:?}", decoded_shared_secret);
    println!("verify_token:  {:?}", decoded_verify_token);

    let mut key = [0u8; 16];
    for (i, x) in decoded_shared_secret[..16].iter().enumerate() {
        key[i] = *x;
    }

    let server_hash =
        mojang_api::server_hash("", key, &state.rsa.as_ref().unwrap().public_key_to_der()?);
    let auth_result = mojang_api::server_auth(&server_hash, &state.name.as_ref().unwrap());

    let mut rt = tokio::runtime::Runtime::new()?;
    let (name, uuid, props) = match rt.block_on(auth_result) {
        Ok(auth) => (auth.name, auth.id, auth.properties),
        Err(e) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("auth failed {:?}", e),
            ))
        }
    };

    println!(
        "name: {}, id: {}, props: {:?}, key: {:?}",
        name, uuid, props, key
    );
    stream.set_decryptor(&key);
    stream.set_encryptor(&key);

    set_compression(stream)?;
    state.name = Some(name.clone());
    login_success(stream, uuid.to_string(), name)?;
    state.uuid = Some(uuid);
    state.mode = Mode::Play;

    Ok(())
}

pub fn disconnect(stream: &mut McStream) -> Result<(), Error> {
    let disconnect = Disconnect {
        chat: r#"{"text": "konaide ( ; _ ; )"}"#.to_string(),
    };
    disconnect.packet_write(stream)?;

    println!("disconnected");
    Ok(())
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

pub fn set_compression(stream: &mut McStream) -> Result<(), Error> {
    let set_compression = SetCompression {
        thresshold: (-1 as i32).into(), // this mean do not compression
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
