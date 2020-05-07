use std::io::{self, Error, Write};
use std::net::TcpStream;

use super::types::string::encode_string;
use super::types::varint::encode_varint;

use super::packet::{read_login_packet, LoginPacket};

use uuid::Uuid;

use openssl::pkey::Private;
use openssl::rsa::{Rsa, Padding};

pub fn disconnect(stream: &mut TcpStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);

    encode_varint(&0, &mut r)?; // packet_id: 0
    encode_string(&r#"{"text": "konaide ( ; _ ; )"}"#.to_string(), &mut r)?;

    encode_varint(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;

    println!("disconnected");
    Ok(())
}

pub fn login_start(stream: &mut TcpStream) -> Result<String, Error> {
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
    stream: &mut TcpStream,
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

    println!("sent encrypted request");
    Ok(())
}

pub fn encryption_response(stream: &mut TcpStream, rsa: &Rsa<Private>, name: &String) -> Result<(), Error> {
    println!("receive encryption response");
    match read_login_packet(stream)? {
        LoginPacket::EncryptionResponse {
            shared_secret,
            verify_token,
            ..
        } => {
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
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    };
    Ok(())
}

pub fn login_success(stream: &mut TcpStream, uuid: &Uuid, username: &String) -> Result<(), Error> {
    stream.write_all(uuid.as_bytes())?;
    encode_string(username, stream)?;

    println!("login successful");
    Ok(())
}
