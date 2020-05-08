use std::io::{self, Error, Write};

use super::mcstream::McStream;
use crate::packet::{read_play_packet, serverbound::PlayPacket};
use crate::protocol::Protocol;
use crate::types::Var;

pub fn join_game(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x26, r_ref)?; // packet id: 0x26
    
    i32::proto_encode(&0, r_ref)?; // entity id
    u8::proto_encode(&1, r_ref)?; // gamemode
    i32::proto_encode(&0, r_ref)?; // dimension
    u64::proto_encode(&0, r_ref)?; // hashed seed
    u8::proto_encode(&2, r_ref)?; // max players
    String::proto_encode(&"flat".to_string(), r_ref)?; // level type
    <Var<i32>>::proto_encode(&1, r_ref)?; // view distance
    bool::proto_encode(&true, r_ref)?; // reduced debug info
    bool::proto_encode(&true, r_ref)?; // enable respawn info

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn client_settings(stream: &mut McStream) -> Result<(), Error> {
    match read_play_packet(stream)? {
        PlayPacket::ClientSettings {
            locale,
            view_distance,
            chat_mode,
            chat_colors,
            displayed_skin_parts,
            main_hand,
        } => {
            println!(
                "locale: {}, view_distance: {}, chat_mode: {}, chat_colors: {}, displayed_skin_parts: {}, main_hand: {}",
                locale, view_distance, chat_mode, chat_colors, displayed_skin_parts, main_hand
            );
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid packet id",
            ))
        }
    }

    Ok(())
}
