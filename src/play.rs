use std::io::{self, Error, Write};

use super::mcstream::McStream;
use crate::packet::{read_play_packet, serverbound::PlayPacket};
use crate::protocol::Protocol;
use crate::types::{Arr, Var};

pub fn join_game(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x26).proto_encode(r_ref)?; // packet id: 0x26

    i32::proto_encode(&0, r_ref)?; // entity id
    u8::proto_encode(&1, r_ref)?; // gamemode
    i32::proto_encode(&0, r_ref)?; // dimension
    u64::proto_encode(&0, r_ref)?; // hashed seed
    u8::proto_encode(&2, r_ref)?; // max players
    String::proto_encode(&"flat".to_string(), r_ref)?; // level type
    Var(0x1).proto_encode(r_ref)?; // view distance
    bool::proto_encode(&true, r_ref)?; // reduced debug info
    bool::proto_encode(&true, r_ref)?; // enable respawn info

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
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

    Var(0x40).proto_encode(r_ref)?; // packet id: 0x40

    u8::proto_encode(&0, r_ref)?; // slot

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn declare_recipes(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x5B).proto_encode(r_ref)?; // packet id: 0x5B

    Var(0).proto_encode(r_ref)?; // num recipes (zero)

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn tags(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x5C).proto_encode(r_ref)?; // packet id: 0x5C

    Var(0).proto_encode(r_ref)?; // num block tags (zero)
    Var(0).proto_encode(r_ref)?; // num item tags (zero)
    Var(0).proto_encode(r_ref)?; // num fluid tags (zero)
    Var(0).proto_encode(r_ref)?; // num entity tags (zero)

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn entity_status(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x1C).proto_encode(r_ref)?; // packet id: 0x1C

    i32::proto_encode(&0, r_ref)?; // entity id
    i8::proto_encode(&2, r_ref)?; // entity status

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn decrale_commands(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x12).proto_encode(r_ref)?; // packet id: 0x12

    Var(0).proto_encode(r_ref)?; // count (zero)
    Var(0).proto_encode(r_ref)?; // root index

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn unlock_recipes(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x37).proto_encode(r_ref)?; // packet id: 0x37

    Var(0).proto_encode(r_ref)?; // action
    bool::proto_encode(&false, r_ref)?; // Crafting Recipe Book Open
    bool::proto_encode(&false, r_ref)?; // Crafting Recipe Book Filter Active
    bool::proto_encode(&false, r_ref)?; // Smelting Recipe Book Open
    bool::proto_encode(&false, r_ref)?; // Smelting Recipe Book Filter Active
    Var(0).proto_encode(r_ref)?; // Array size 1
    Var(0).proto_encode(r_ref)?; // Array size 2

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn play_position_and_look(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x36).proto_encode(r_ref)?; // packet id: 0x36

    f64::proto_encode(&0.0, r_ref)?; // X
    f64::proto_encode(&64.0, r_ref)?; // Y
    f64::proto_encode(&0.0, r_ref)?; // Z
    f32::proto_encode(&0.0, r_ref)?; // Yaw
    f32::proto_encode(&0.0, r_ref)?; // Pitch
    u8::proto_encode(&0, r_ref)?; // flags
    Var(0).proto_encode(r_ref)?; // teleport id

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn player_info(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x34).proto_encode(r_ref)?; // packet id: 0x34

    Var(0).proto_encode(r_ref)?; // action
    Var(0).proto_encode(r_ref)?; // number of players

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn update_view_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x41).proto_encode(r_ref)?; // packet id: 0x41

    Var(0).proto_encode(r_ref)?; // chunk x
    Var(0).proto_encode(r_ref)?; // chunk z

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn update_light(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x25).proto_encode(r_ref)?; // packet id: 0x25

    Var(0).proto_encode(r_ref)?; // chunk x
    Var(0).proto_encode(r_ref)?; // chunk z
    Var(3).proto_encode(r_ref)?; // sky light mask
    Var(0).proto_encode(r_ref)?; // block light mask
    Var(0).proto_encode(r_ref)?; // empty sky light mask
    Var(0).proto_encode(r_ref)?; // empty block light mask
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // sky light arrays
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // block light arrays

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn chunk_data(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x22).proto_encode(r_ref)?; // packet id: 0x22

    i32::proto_encode(&0, r_ref)?; // chunk x
    i32::proto_encode(&0, r_ref)?; // chunk z
    bool::proto_encode(&true, r_ref)?; // full chunk
    Var(3).proto_encode(r_ref)?; // primary bit mask
    <Arr<Var<i32>, i64>>::proto_encode(&[0; 16 * 16 * 16].to_vec(), &mut r)?; // heightmaps
    <Arr<Var<i32>, u8>>::proto_encode(&[0; 4 * 4 * 4].to_vec(), &mut r)?; // data
    <Arr<Var<i32>, i32>>::proto_encode(&[].to_vec(), &mut r)?; // block entities

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn world_border(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x3E).proto_encode(r_ref)?; // packet id: 0x3E

    Var(0).proto_encode(r_ref)?; // action
    f64::proto_encode(&16.0, r_ref)?; // diameter

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn spawn_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    Var(0x4E).proto_encode(r_ref)?; // packet id: 0x4E

    u64::proto_encode(
        &(((0 & 0x3FFFFFFu64) << 38) | ((0 & 0x3FFFFFFu64) << 12) | (64 & 0xFFF) as u64),
        r_ref,
    )?; // location

    Var(r.get_ref().len() as i32).proto_encode(stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}
