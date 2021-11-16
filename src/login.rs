use std::io::{self, Error};
use std::str::FromStr;

use crate::packet::client::{self, EncryptionRequest, LoginSuccess, SetCompression};
use crate::packet::server::{EncryptionResponse, LoginStart};
use crate::server::Worker;

use openssl::rsa::Padding;
use uuid::Uuid;

pub async fn crack_login_start(worker: &mut Worker, _login_start: LoginStart) -> Result<(), Error> {
    // crack
    let uuid = Uuid::from_str("af6c5ee2-8eeb-8099-b8ce-253c50b0d8a8").unwrap();
    login_success(worker, uuid.to_string(), "todo".to_string()).await?;
    worker.state.uuid = Some(uuid);
    worker.state.name = Some("todo".to_string());
    worker.state.crack = true;

    return Ok(());
}

pub async fn login_start(worker: &mut Worker, login_start: LoginStart) -> Result<(), Error> {
    worker.state.rsa = Some(openssl::rsa::Rsa::generate(1024).unwrap());
    let public_key = worker
        .state
        .rsa
        .as_ref()
        .expect("maybe not in login mode")
        .public_key_to_der()?;
    let verify_token = vec![0u8, 123, 212, 123];
    println!("login attempt: {}", login_start.name);
    worker.state.name = Some(login_start.name);
    encryption_request(worker, public_key, verify_token).await?;

    return Ok(());
}

pub async fn encryption_response(
    worker: &mut Worker,
    encryption_response: EncryptionResponse,
) -> Result<(), Error> {
    println!("receive encryption response");
    // use mojang_api::ServerAuthResponse;

    let mut decoded_shared_secret = vec![0; worker.state.rsa.as_ref().unwrap().size() as usize];
    worker.state.rsa.as_ref().unwrap().private_decrypt(
        &encryption_response.shared_secret,
        &mut decoded_shared_secret,
        Padding::PKCS1,
    )?;
    let mut decoded_verify_token = vec![0; worker.state.rsa.as_ref().unwrap().size() as usize];
    worker.state.rsa.as_ref().unwrap().private_decrypt(
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

    let server_hash = mojang_api::server_hash(
        "",
        key,
        &worker.state.rsa.as_ref().unwrap().public_key_to_der()?,
    );
    let auth_result =
        mojang_api::server_auth(&server_hash, &worker.state.name.as_ref().unwrap()).await;

    let (name, uuid, props) = match auth_result {
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
    worker.set_key(&key);

    set_compression(worker).await?;
    worker.state.name = Some(name.clone());
    login_success(worker, uuid.to_string(), name).await?;
    worker.state.uuid = Some(uuid);

    Ok(())
}

// pub fn disconnect(stream: &mut McStream) -> Result<(), Error> {
//     let disconnect = Disconnect {
//         chat: r#"{"text": "konaide ( ; _ ; )"}"#.to_string(),
//     };
//     disconnect.packet_write(stream)?;

//     println!("disconnected");
//     Ok(())
// }

pub async fn encryption_request(
    worker: &mut Worker,
    public_key: Vec<u8>,
    verify_token: Vec<u8>,
) -> Result<(), Error> {
    let packet = client::LoginPacket::EncryptionRequest(EncryptionRequest {
        server_id: "".to_string(),
        public_key,
        verify_token,
    });
    worker.write_packet(packet).await?;

    println!("sent encrypted request");
    Ok(())
}

pub async fn set_compression(worker: &mut Worker) -> Result<(), Error> {
    let packet = client::LoginPacket::SetCompression(SetCompression {
        thresshold: (-1 as i32).into(), // this mean do not compression
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn login_success(
    worker: &mut Worker,
    uuid: String,
    username: String,
) -> Result<(), Error> {
    let packet = client::LoginPacket::LoginSuccess(LoginSuccess { uuid, username });
    worker.write_packet(packet).await?;

    println!("login successful");
    Ok(())
}
