use std::io::{self, Error, Write};
use std::net::TcpStream;

use super::types::string::encode_string;
use super::types::varint::encode_varint;

use super::packet::{read_login_packet, LoginPacket};

use uuid::Uuid;

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

pub fn login_success(stream: &mut TcpStream, uuid: &Uuid, username: &String) -> Result<(), Error> {
    stream.write_all(uuid.as_bytes())?;
    encode_string(username, stream)?;

    println!("login successful");
    Ok(())
}