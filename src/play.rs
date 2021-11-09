use std::io::{self, Error};

use super::mcstream::McStream;
use crate::packet::client::{
    AddPlayer, ChunkData, DeclareCommands, DeclareRecipes, EntityStatus, HeldItemChange, JoinGame,
    PlayerInfo, PlayerInfoAction, PlayerPositionAndLook, SpawnPosition, Tags, UnlockRecipes,
    UpdateLight, UpdateViewPosition, WorldBorder, WorldBorderAction,
};
use crate::packet::PacketWrite;
use crate::packet::{read_play_packet, server::PlayPacket};
use crate::protocol::ProtocolWrite;
use crate::state::State;
use crate::types::chunk_section::ChunkSection;
use crate::types::heightmap::Heightmaps;
use crate::types::nbt::Nbt;
use crate::types::position::Position;

pub fn join_game(stream: &mut McStream) -> Result<(), Error> {
    let join_game = JoinGame {
        entity_id: 0,
        game_mode: 1,
        dimension: 0,
        hashed_seed: 0,
        max_players: 3,
        level_type: "flat".to_owned(),
        view_distance: 1.into(),
        reduced_debug_info: false,
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
        flags: 0,
        teleport_id: 0.into(),
    };
    play_position_and_look.packet_write(stream)?;

    Ok(())
}

pub fn player_info(stream: &mut McStream, state: &mut State) -> Result<(), Error> {
    let player_info = PlayerInfo {
        action: PlayerInfoAction::AddPlayer(vec![AddPlayer {
            uuid: state.uuid.as_ref().unwrap().clone(),
            name: state.name.as_ref().unwrap().to_string(),
            props: vec![
                // Properties {
                //     name: "test".to_owned(),
                //     value: "var".to_owned(),
                //     is_signed: true,
                //     signature: Some("yoyo".to_owned()),
                // }
            ],
            gamemode: 0.into(),
            ping: 1.into(),
            has_display_name: false,
            display_name: None,
        }]),
    };
    player_info.packet_write(stream)?;

    Ok(())
}

pub fn update_view_position(stream: &mut McStream) -> Result<(), Error> {
    let update_view_position = UpdateViewPosition {
        chunk_x: 0.into(),
        chunk_z: 1.into(),
    };
    update_view_position.packet_write(stream)?;

    Ok(())
}

pub fn update_light(stream: &mut McStream) -> Result<(), Error> {
    let update_light = UpdateLight {
        chunk_x: 0.into(),
        chunk_z: 0.into(),
        sky_light_mask: 3.into(),
        block_light_mask: 0.into(),
        empty_sky_light_mask: 0.into(),
        empty_block_light_mask: 0.into(),
        sky_lights: vec![127; 2048],
        block_lights: vec![127; 2048],
    };
    update_light.packet_write(stream)?;

    Ok(())
}

pub fn chunk_data(stream: &mut McStream) -> Result<(), Error> {
    for x in 0..4 {
        for z in 0..4 {
            let chunk_section = ChunkSection::from_array_and_palette(
                &[2; 4096],
                vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()],
            );
            let air_chunk_section = ChunkSection::from_array_and_palette(
                &[0; 4096],
                vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()],
            );
            let mut data = vec![];
            ChunkSection::proto_encode(&chunk_section, &mut data)?;
            for _ in 0..15 {
                ChunkSection::proto_encode(&air_chunk_section, &mut data)?;
            }

            let chunk_data = ChunkData {
                chunk_x: x,
                chunk_z: z,
                full_chunk: true,
                primary_bit_mask: 0b1111111111111111.into(),
                heightmaps: Nbt(Heightmaps::from_array(&[16; 256])),
                biomes: Some(vec![127.into(); 1024]),
                data,
                block_entities: vec![],
            };
            chunk_data.packet_write(stream)?;
        }
    }

    Ok(())
}

pub fn world_border(stream: &mut McStream) -> Result<(), Error> {
    let world_border = WorldBorder {
        action: WorldBorderAction::SetSize { diameter: 32.0 },
    };
    world_border.packet_write(stream)?;

    Ok(())
}

pub fn spawn_position(stream: &mut McStream) -> Result<(), Error> {
    let spawn_position = SpawnPosition {
        location: Position { x: 0, y: 64, z: 0 },
    };
    spawn_position.packet_write(stream)?;

    Ok(())
}
