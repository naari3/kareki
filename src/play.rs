use std::io::{self, Error, Write};

use super::mcstream::McStream;
use crate::packet::{read_play_packet, serverbound::PlayPacket};
use crate::protocol::Protocol;
use crate::types::{Arr, Var};

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

pub fn held_item_change(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x40, r_ref)?; // packet id: 0x40

    u8::proto_encode(&0, r_ref)?; // slot

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn declare_recipes(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x5B, r_ref)?; // packet id: 0x5B

    <Var<i32>>::proto_encode(&0, r_ref)?; // num recipes (zero)

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn tags(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x5C, r_ref)?; // packet id: 0x5C

    <Var<i32>>::proto_encode(&0, r_ref)?; // num block tags (zero)
    <Var<i32>>::proto_encode(&0, r_ref)?; // num item tags (zero)
    <Var<i32>>::proto_encode(&0, r_ref)?; // num fluid tags (zero)
    <Var<i32>>::proto_encode(&0, r_ref)?; // num entity tags (zero)

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn entity_status(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x1C, r_ref)?; // packet id: 0x1C

    i32::proto_encode(&0, r_ref)?; // entity id
    i8::proto_encode(&2, r_ref)?; // entity status

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn decrale_commands(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x12, r_ref)?; // packet id: 0x12

    <Var<i32>>::proto_encode(&0, r_ref)?; // count (zero)
    <Var<i32>>::proto_encode(&0, r_ref)?; // root index

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn unlock_recipes(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x37, r_ref)?; // packet id: 0x37

    <Var<i32>>::proto_encode(&0, r_ref)?; // action
    bool::proto_encode(&false, r_ref)?; // Crafting Recipe Book Open
    bool::proto_encode(&false, r_ref)?; // Crafting Recipe Book Filter Active
    bool::proto_encode(&false, r_ref)?; // Smelting Recipe Book Open
    bool::proto_encode(&false, r_ref)?; // Smelting Recipe Book Filter Active
    <Var<i32>>::proto_encode(&0, r_ref)?; // Array size 1
    <Var<i32>>::proto_encode(&0, r_ref)?; // Array size 2

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn play_position_and_look(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x36, r_ref)?; // packet id: 0x36

    f64::proto_encode(&0.0, r_ref)?; // X
    f64::proto_encode(&64.0, r_ref)?; // Y
    f64::proto_encode(&0.0, r_ref)?; // Z
    f32::proto_encode(&0.0, r_ref)?; // Yaw
    f32::proto_encode(&0.0, r_ref)?; // Pitch
    u8::proto_encode(&0, r_ref)?; // flags
    <Var<i32>>::proto_encode(&0, r_ref)?; // teleport id

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn player_info(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x34, r_ref)?; // packet id: 0x34

    <Var<i32>>::proto_encode(&0, r_ref)?; // action
    <Var<i32>>::proto_encode(&0, r_ref)?; // number of players

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn update_view_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x41, r_ref)?; // packet id: 0x41

    <Var<i32>>::proto_encode(&0, r_ref)?; // chunk x
    <Var<i32>>::proto_encode(&0, r_ref)?; // chunk z

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn update_light(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x25, r_ref)?; // packet id: 0x25

    <Var<i32>>::proto_encode(&0, r_ref)?; // chunk x
    <Var<i32>>::proto_encode(&0, r_ref)?; // chunk z
    <Var<i32>>::proto_encode(&3, r_ref)?; // sky light mask
    <Var<i32>>::proto_encode(&0, r_ref)?; // block light mask
    <Var<i32>>::proto_encode(&0, r_ref)?; // empty sky light mask
    <Var<i32>>::proto_encode(&0, r_ref)?; // empty block light mask
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // sky light arrays
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // block light arrays

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn chunk_data(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x22, r_ref)?; // packet id: 0x22

    i32::proto_encode(&0, r_ref)?; // chunk x
    i32::proto_encode(&0, r_ref)?; // chunk z
    bool::proto_encode(&true, r_ref)?; // full chunk
    <Var<i32>>::proto_encode(&3, r_ref)?; // primary bit mask
    <Arr<Var<i32>, i64>>::proto_encode(&[0; 16 * 16 * 16].to_vec(), &mut r)?; // heightmaps
    <Arr<Var<i32>, u8>>::proto_encode(&[0; 4 * 4 * 4].to_vec(), &mut r)?; // data
    <Arr<Var<i32>, i32>>::proto_encode(&[].to_vec(), &mut r)?; // block entities

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn world_border(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x3E, r_ref)?; // packet id: 0x3E

    <Var<i32>>::proto_encode(&0, r_ref)?; // action
    f64::proto_encode(&16.0, r_ref)?; // diameter

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn spawn_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x4E, r_ref)?; // packet id: 0x4E

    u64::proto_encode(
        &(((0 & 0x3FFFFFFu64) << 38) | ((0 & 0x3FFFFFFu64) << 12) | (64 & 0xFFF) as u64),
        r_ref,
    )?; // location

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}
