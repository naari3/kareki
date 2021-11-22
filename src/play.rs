use std::io::Result;

use crate::client::Client;
use crate::packet::client::{
    AddPlayer, DeclareCommands, DeclareRecipes, EntityStatus, HeldItemChange, JoinGame, PlayPacket,
    PlayerInfo, PlayerInfoAction, PlayerPositionAndLook, SpawnPosition, Tags, UnlockRecipes,
    UpdateViewPosition, WorldBorder, WorldBorderAction,
};

use crate::types::position::Position;

pub fn join_game(client: &mut Client) -> Result<()> {
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

    client.send_play_packet(packet)?;

    Ok(())
}

pub fn held_item_change(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::HeldItemChange(HeldItemChange { slot: 0 });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn declare_recipes(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::DeclareRecipes(DeclareRecipes { recipes: vec![] });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn tags(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::Tags(Tags {
        block_tags: vec![],
        item_tags: vec![],
        fluid_tags: vec![],
        entity_tags: vec![],
    });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn entity_status(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::EntityStatus(EntityStatus {
        entity_id: 0,
        entity_status: 2,
    });
    client.send_play_packet(packet)?;

    Ok(())
}

#[allow(dead_code)]
pub fn declare_commands(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::DeclareCommands(DeclareCommands {
        nodes: vec![],
        root_index: 0.into(),
    });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn unlock_recipes(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::UnlockRecipes(UnlockRecipes {
        action: 0.into(),
        crafting_recipe_book_open: true,
        crafting_recipe_book_filter_active: false,
        smelting_recipe_book_open: false,
        smelting_recipe_book_filter_active: false,
        recipe_ids: vec![],
        additional_recipe_ids: Some(vec![]),
    });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn play_position_and_look(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::PlayerPositionAndLook(PlayerPositionAndLook {
        x: 0.0,
        y: 64.0,
        z: 0.0,
        yaw: 0.0,
        pitch: 0.0,
        flags: 0,
        teleport_id: 0.into(),
    });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn player_info(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::PlayerInfo(PlayerInfo {
        action: PlayerInfoAction::AddPlayer(vec![AddPlayer {
            uuid: client.state.uuid.as_ref().unwrap().clone(),
            name: client.state.name.as_ref().unwrap().to_string(),
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
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn update_view_position(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::UpdateViewPosition(UpdateViewPosition {
        chunk_x: 0.into(),
        chunk_z: 1.into(),
    });
    client.send_play_packet(packet)?;

    Ok(())
}

#[allow(dead_code)]
pub fn world_border(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::WorldBorder(WorldBorder {
        action: WorldBorderAction::SetSize { diameter: 128.0 },
    });
    client.send_play_packet(packet)?;

    Ok(())
}

pub fn spawn_position(client: &mut Client) -> Result<()> {
    let packet = PlayPacket::SpawnPosition(SpawnPosition {
        location: Position { x: 0, y: 64, z: 0 },
    });
    client.send_play_packet(packet)?;

    Ok(())
}
