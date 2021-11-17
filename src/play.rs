use std::io::{self, Result};

use crate::packet::client::{
    AddPlayer, ChunkData, DeclareCommands, DeclareRecipes, EntityStatus, HeldItemChange, JoinGame,
    PlayPacket, PlayerInfo, PlayerInfoAction, PlayerPositionAndLook, SpawnPosition, Tags,
    UnlockRecipes, UpdateLight, UpdateViewPosition, WorldBorder, WorldBorderAction,
};
use crate::packet::server;
use crate::protocol::ProtocolWrite;
use crate::server::Worker;
use crate::types::chunk::Chunk;
use crate::types::chunk_section::ChunkSection;
use crate::types::heightmap::Heightmaps;
use crate::types::nbt::Nbt;
use crate::types::position::Position;

pub async fn join_game(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::JoinGame(JoinGame {
        entity_id: 0,
        game_mode: 1,
        dimension: 0,
        hashed_seed: 0,
        max_players: 3,
        level_type: "flat".to_owned(),
        view_distance: 4.into(),
        reduced_debug_info: false,
        enable_respawn_screen: true,
    });

    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn client_settings(worker: &mut Worker) -> Result<()> {
    match worker.read_packet_exact().await? {
        server::PlayPacket::ClientSettings(client_settings) => {
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

pub async fn held_item_change(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::HeldItemChange(HeldItemChange { slot: 0 });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn declare_recipes(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::DeclareRecipes(DeclareRecipes { recipes: vec![] });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn tags(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::Tags(Tags {
        block_tags: vec![],
        item_tags: vec![],
        fluid_tags: vec![],
        entity_tags: vec![],
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn entity_status(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::EntityStatus(EntityStatus {
        entity_id: 0,
        entity_status: 2,
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn declare_commands(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::DeclareCommands(DeclareCommands {
        nodes: vec![],
        root_index: 0.into(),
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn unlock_recipes(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::UnlockRecipes(UnlockRecipes {
        action: 0.into(),
        crafting_recipe_book_open: true,
        crafting_recipe_book_filter_active: false,
        smelting_recipe_book_open: false,
        smelting_recipe_book_filter_active: false,
        recipe_ids: vec![],
        additional_recipe_ids: Some(vec![]),
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn play_position_and_look(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::PlayerPositionAndLook(PlayerPositionAndLook {
        x: 0.0,
        y: 64.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
        teleport_id: 0.into(),
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn player_info(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::PlayerInfo(PlayerInfo {
        action: PlayerInfoAction::AddPlayer(vec![AddPlayer {
            uuid: worker.state.uuid.as_ref().unwrap().clone(),
            name: worker.state.name.as_ref().unwrap().to_string(),
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
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn update_view_position(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::UpdateViewPosition(UpdateViewPosition {
        chunk_x: 0.into(),
        chunk_z: 1.into(),
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn update_light(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::UpdateLight(UpdateLight {
        chunk_x: 0.into(),
        chunk_z: 0.into(),
        sky_light_mask: 3.into(),
        block_light_mask: 0.into(),
        empty_sky_light_mask: 0.into(),
        empty_block_light_mask: 0.into(),
        sky_lights: vec![127; 2048],
        block_lights: vec![127; 2048],
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn chunk_data(worker: &mut Worker) -> Result<()> {
    let mut chunk = Chunk::empty();
    for x in 0..16 {
        for z in 0..16 {
            for y in 0..16 {
                if (x + y + z) / 4 % 3 != 0 {
                    chunk.set_block(x, y, z, (((x + y + z) % 8) + 1) as u16)?;
                }
            }
        }
    }
    for x in -2..2 {
        for z in -2..2 {
            let mut data = vec![];
            for section in chunk.sections.iter() {
                ChunkSection::proto_encode(section, &mut data)?;
            }

            let packet = PlayPacket::ChunkData(ChunkData {
                chunk_x: x,
                chunk_z: z,
                full_chunk: true,
                primary_bit_mask: 0b1111111111111111.into(),
                heightmaps: Nbt(Heightmaps::from_array(&[0; 256])),
                biomes: Some(vec![0.into(); 1024]),
                data,
                block_entities: vec![],
            });
            worker.write_packet(packet).await?;
        }
    }

    Ok(())
}

pub async fn world_border(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::WorldBorder(WorldBorder {
        action: WorldBorderAction::SetSize { diameter: 128.0 },
    });
    worker.write_packet(packet).await?;

    Ok(())
}

pub async fn spawn_position(worker: &mut Worker) -> Result<()> {
    let packet = PlayPacket::SpawnPosition(SpawnPosition {
        location: Position { x: 0, y: 64, z: 0 },
    });
    worker.write_packet(packet).await?;

    Ok(())
}
