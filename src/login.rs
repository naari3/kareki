use std::io::{self, Error, Write};

use super::mcstream::McStream;

use super::types::string::encode_string;
use super::types::varint::encode_varint;

use super::packet::{read_login_packet, LoginPacket};

use uuid::Uuid;

use openssl::pkey::Private;
use openssl::rsa::{Padding, Rsa};

pub fn disconnect(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);

    encode_varint(&0, &mut r)?; // packet_id: 0
    encode_string(&r#"{"text": "konaide ( ; _ ; )"}"#.to_string(), &mut r)?;

    encode_varint(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;

    println!("disconnected");
    Ok(())
}

pub fn login_start(stream: &mut McStream) -> Result<String, Error> {
    match read_login_packet(stream)? {
        LoginPacket::LoginStart { name } => {
            return Ok(name);
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }
}

pub fn encryption_request(
    stream: &mut McStream,
    pubkey: &Vec<u8>,
    verify_token: &Vec<u8>,
) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let mut dst = io::Cursor::new(vec![] as Vec<u8>);

    encode_varint(&1, &mut r)?; // packet_id: 1

    encode_string(&"".to_string(), &mut r)?;
    encode_varint(&(pubkey.len() as i32), &mut r)?;
    r.write(&pubkey)?;
    encode_varint(&(verify_token.len() as i32), &mut r)?;
    r.write(&verify_token)?;

    encode_varint(&(r.get_ref().len() as i32), &mut dst)?;
    dst.write_all(r.get_ref())?;

    stream.write_all(dst.get_ref())?;
    stream.flush()?;

    println!("sent encrypted request");
    Ok(())
}

pub fn encryption_response(
    stream: &mut McStream,
    rsa: &Rsa<Private>,
    name: &String,
) -> Result<(String, Uuid, Vec<mojang_api::ProfileProperty>, [u8; 16]), Error> {
    println!("receive encryption response");
    match read_login_packet(stream)? {
        LoginPacket::EncryptionResponse {
            shared_secret,
            verify_token,
            ..
        } => {
            // use mojang_api::ServerAuthResponse;

            let mut decoded_shared_secret = vec![0; rsa.size() as usize];
            rsa.private_decrypt(&shared_secret, &mut decoded_shared_secret, Padding::PKCS1)?;
            let mut decoded_verify_token = vec![0; rsa.size() as usize];
            rsa.private_decrypt(&verify_token, &mut decoded_verify_token, Padding::PKCS1)?;
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
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    };
}

pub fn set_compression(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    encode_varint(&3, &mut r)?; // packet_id: 3
    encode_varint(&-1, &mut r)?; // this mean do not compression

    encode_varint(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    println!("login successful");
    Ok(())
}

pub fn login_success(stream: &mut McStream, uuid: &Uuid, username: &String) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    encode_varint(&2, &mut r)?; // packet_id: 2

    encode_string(&uuid.to_string(), &mut r)?;

    encode_string(username, &mut r)?;

    encode_varint(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    println!("login successful");
    Ok(())
}
