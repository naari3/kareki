use std::io::{self, Error, Write};
use std::str::FromStr;

use uuid::Uuid;

use super::mcstream::McStream;
use crate::packet::client::{
    AddPlayer, DeclareCommands, DeclareRecipes, EntityStatus, HeldItemChange, JoinGame, PlayerInfo,
    PlayerInfoAction, PlayerPositionAndLook, Properties, Tags, UnlockRecipes,
};
use crate::packet::PacketWrite;
use crate::packet::{read_play_packet, server::PlayPacket};
use crate::protocol::ProtocolWrite;
use crate::types::{Arr, Var};

pub fn join_game(stream: &mut McStream) -> Result<(), Error> {
    let join_game = JoinGame {
        entity_id: 0,
        game_mode: 1,
        dimension: 0,
        hashed_seed: 0,
        max_players: 3,
        level_type: "flat".to_owned(),
        view_distance: 1.into(),
        reduced_debug_info: true,
        enable_respawn_screen: true,
    };

    join_game.packet_write(stream)?;

    Ok(())
}

pub fn client_settings(stream: &mut McStream) -> Result<(), Error> {
    match read_play_packet(stream)? {
        PlayPacket::ClientSettings(client_settings) => {
            println!(
                "locale: {}, view_distance: {}, chat_mode: {}, chat_colors: {}, displayed_skin_parts: {}, main_hand: {}",
                client_settings.locale, client_settings.view_distance, client_settings.chat_mode, client_settings.chat_colors, client_settings.displayed_skin_parts, client_settings.main_hand
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
    let held_item_change = HeldItemChange { slot: 0 };
    held_item_change.packet_write(stream)?;

    Ok(())
}

pub fn declare_recipes(stream: &mut McStream) -> Result<(), Error> {
    let declare_recipes = DeclareRecipes { recipes: vec![] };
    declare_recipes.packet_write(stream)?;

    Ok(())
}

pub fn tags(stream: &mut McStream) -> Result<(), Error> {
    let tags = Tags {
        block_tags: vec![],
        item_tags: vec![],
        fluid_tags: vec![],
        entity_tags: vec![],
    };
    tags.packet_write(stream)?;

    Ok(())
}

pub fn entity_status(stream: &mut McStream) -> Result<(), Error> {
    let entity_status = EntityStatus {
        entity_id: 0,
        entity_status: 2,
    };
    entity_status.packet_write(stream)?;

    Ok(())
}

pub fn declare_commands(stream: &mut McStream) -> Result<(), Error> {
    let declare_commands = DeclareCommands {
        nodes: vec![],
        root_index: 0.into(),
    };
    declare_commands.packet_write(stream)?;

    Ok(())
}

pub fn unlock_recipes(stream: &mut McStream) -> Result<(), Error> {
    let unlock_recipes = UnlockRecipes {
        action: 0.into(),
        crafting_recipe_book_open: true,
        crafting_recipe_book_filter_active: false,
        smelting_recipe_book_open: false,
        smelting_recipe_book_filter_active: false,
        recipe_ids: vec![],
        additional_recipe_ids: Some(vec![]),
    };
    unlock_recipes.packet_write(stream)?;

    Ok(())
}

pub fn play_position_and_look(stream: &mut McStream) -> Result<(), Error> {
    let play_position_and_look = PlayerPositionAndLook {
        x: 0.0,
        y: 64.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0x1F,
        teleport_id: 0.into(),
    };
    play_position_and_look.packet_write(stream)?;

    Ok(())
}

pub fn player_info(stream: &mut McStream) -> Result<(), Error> {
    let player_info = PlayerInfo {
        action: 0.into(),
        player: PlayerInfoAction::AddPlayer(vec![AddPlayer {
            uuid: Uuid::from_str("7e713126-452c-40e7-9374-c9333d3502ed")
                .expect("Expected valid uuid"),
            name: "todo".to_owned(),
            props: vec![Properties {
                name: "test".to_owned(),
                value: "var".to_owned(),
                is_signed: false,
                signature: None,
            }],
            gamemode: 0.into(),
            ping: 1.into(),
            has_display_name: true,
            display_name: Some("yoyo".to_owned()),
        }]),
    };
    player_info.packet_write(stream)?;

    Ok(())
}

pub fn update_view_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x41.into(), r_ref)?; // packet id: 0x41

    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // chunk x
    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // chunk z

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn update_light(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x25.into(), r_ref)?; // packet id: 0x25

    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // chunk x
    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // chunk z
    <Var<i32>>::proto_encode(&3.into(), r_ref)?; // sky light mask
    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // block light mask
    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // empty sky light mask
    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // empty block light mask
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // sky light arrays
    <Arr<Var<i32>, u8>>::proto_encode(&[127u8; 2048].to_vec(), &mut r)?; // block light arrays

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn chunk_data(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x22.into(), r_ref)?; // packet id: 0x22

    i32::proto_encode(&0, r_ref)?; // chunk x
    i32::proto_encode(&0, r_ref)?; // chunk z
    bool::proto_encode(&true, r_ref)?; // full chunk
    <Var<i32>>::proto_encode(&3.into(), r_ref)?; // primary bit mask
    <Arr<Var<i32>, i64>>::proto_encode(&[0; 16 * 16 * 16].to_vec(), &mut r)?; // heightmaps
    <Arr<Var<i32>, u8>>::proto_encode(&[0; 4 * 4 * 4].to_vec(), &mut r)?; // data
    <Arr<Var<i32>, i32>>::proto_encode(&[].to_vec(), &mut r)?; // block entities

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn world_border(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x3E.into(), r_ref)?; // packet id: 0x3E

    <Var<i32>>::proto_encode(&0.into(), r_ref)?; // action
    f64::proto_encode(&16.0, r_ref)?; // diameter

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}

pub fn spawn_position(stream: &mut McStream) -> Result<(), Error> {
    let mut r = io::Cursor::new(vec![] as Vec<u8>);
    let r_ref = &mut r;

    <Var<i32>>::proto_encode(&0x4E.into(), r_ref)?; // packet id: 0x4E

    u64::proto_encode(
        &(((0 & 0x3FFFFFFu64) << 38) | ((0 & 0x3FFFFFFu64) << 12) | (64 & 0xFFF) as u64),
        r_ref,
    )?; // location

    <Var<i32>>::proto_encode(&(r.get_ref().len() as i32).into(), stream)?;
    stream.write_all(r.get_ref())?;
    stream.flush()?;

    Ok(())
}
