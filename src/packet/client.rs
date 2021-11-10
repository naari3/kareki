use std::io::{self, Write};

use uuid::Uuid;

use crate::{
    protocol::ProtocolWrite,
    types::{
        block_entity::BlockEntity, heightmap::Heightmaps, nbt::Nbt, position::Position, Arr, Var,
    },
};

use super::PacketWrite;
pub enum _StatusPacket {
    SlpResponse(SlpResponse),
    Pong(Pong),
}

#[derive(Debug, Clone)]
pub struct SlpResponse {
    pub json_response: String,
}

impl PacketWrite for SlpResponse {}

impl ProtocolWrite for SlpResponse {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0.into(), dst)?; // packet_id: 0

        String::proto_encode(&value.json_response, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Pong {
    pub payload: u64,
}

impl PacketWrite for Pong {}

impl ProtocolWrite for Pong {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&1.into(), dst)?; // packet_id: 1

        u64::proto_encode(&value.payload, dst)?;
        Ok(())
    }
}

pub enum _Login {
    Disconnect(Disconnect),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
}

#[derive(Debug, Clone)]
pub struct Disconnect {
    pub chat: String,
}

impl PacketWrite for Disconnect {}

impl ProtocolWrite for Disconnect {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0.into(), dst)?; // packet_id: 1

        String::proto_encode(&value.chat, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}
impl PacketWrite for EncryptionRequest {}

impl ProtocolWrite for EncryptionRequest {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&1.into(), dst)?; // packet_id: 1

        String::proto_encode(&value.server_id, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.public_key, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.verify_token, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LoginSuccess {
    pub uuid: String,
    pub username: String,
}
impl PacketWrite for LoginSuccess {}

impl ProtocolWrite for LoginSuccess {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&2.into(), dst)?; // packet_id: 2
        String::proto_encode(&value.uuid.to_string(), dst)?;
        String::proto_encode(&value.username, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SetCompression {
    pub thresshold: Var<i32>,
}
impl PacketWrite for SetCompression {}

impl ProtocolWrite for SetCompression {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&3.into(), dst)?; // packet_id: 3
        <Var<i32>>::proto_encode(&value.thresshold, dst)?;
        Ok(())
    }
}

pub enum _Play {
    DeclareCommands(DeclareCommands),             // 0x12
    EntityStatus(EntityStatus),                   // 0x1C
    KeepAlive(KeepAlive),                         // 0x21
    ChunkData(ChunkData),                         // 0x22
    UpdateLight(UpdateLight),                     // 0x25
    JoinGame(JoinGame),                           // 0x26
    PlayerInfo(PlayerInfo),                       // 0x34
    PlayerPositionAndLook(PlayerPositionAndLook), // 0x36
    UnlockRecipes(UnlockRecipes),                 // 0x37
    WorldBorder(WorldBorder),                     // 0x3E
    HeldItemChange(HeldItemChange),               // 0x40
    UpdateViewPosition(UpdateViewPosition),       // 0x41
    SpawnPosition(SpawnPosition),                 // 0x4E
    DeclareRecipes(DeclareRecipes),               // 0x5B
    Tags(Tags),                                   // 0x5C
}

#[derive(Debug, Clone)]
pub struct DeclareCommands {
    pub nodes: Vec<Node>,
    pub root_index: Var<i32>,
}
impl PacketWrite for DeclareCommands {}

impl ProtocolWrite for DeclareCommands {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x12.into(), dst)?; // packet_id: 0x12
        <Arr<Var<i32>, Node>>::proto_encode(&value.nodes, dst)?;
        <Var<i32>>::proto_encode(&value.root_index, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub flags: u8,
    pub children: Vec<Var<i32>>,
    pub redirect_node: Option<Var<i32>>,
    pub name: Option<String>,
    pub parser: Option<String>,
    // TODO: props
    pub suggestions_type: Option<String>,
}

impl ProtocolWrite for Node {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        u8::proto_encode(&value.flags, dst)?;
        <Arr<Var<i32>, Var<i32>>>::proto_encode(&value.children, dst)?;
        Option::proto_encode(&value.redirect_node, dst)?;
        Option::proto_encode(&value.name, dst)?;
        Option::proto_encode(&value.parser, dst)?;
        Option::proto_encode(&value.suggestions_type, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct EntityStatus {
    pub entity_id: i32,
    pub entity_status: i8,
}
impl PacketWrite for EntityStatus {}

impl ProtocolWrite for EntityStatus {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x1C.into(), dst)?; // packet_id: 0x1C
        i32::proto_encode(&value.entity_id, dst)?;
        i8::proto_encode(&value.entity_status, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct KeepAlive {
    pub keep_alive_id: i64,
}
impl PacketWrite for KeepAlive {}

impl ProtocolWrite for KeepAlive {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x21.into(), dst)?; // packet_id: 0x21
        i64::proto_encode(&value.keep_alive_id, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub full_chunk: bool,
    pub primary_bit_mask: Var<i32>,
    pub heightmaps: Nbt<Heightmaps>,
    pub biomes: Option<Vec<i32>>,
    pub data: Vec<u8>,
    pub block_entities: Vec<Nbt<BlockEntity>>,
}

impl PacketWrite for ChunkData {}

impl ProtocolWrite for ChunkData {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x22.into(), dst)?; // packet_id: 0x22
        i32::proto_encode(&value.chunk_x, dst)?;
        i32::proto_encode(&value.chunk_z, dst)?;
        bool::proto_encode(&value.full_chunk, dst)?;
        <Var<i32>>::proto_encode(&value.primary_bit_mask, dst)?;
        Nbt::proto_encode(&value.heightmaps, dst)?;
        if let Some(biomes) = &value.biomes {
            for biome in biomes {
                i32::proto_encode(biome, dst)?;
            }
        }
        <Arr<Var<i32>, u8>>::proto_encode(&value.data, dst)?;
        // <Var<i32>>::proto_encode(&((value.data.len() as i32).into()), dst)?;
        // dst.write_all(&value.data)?; // maybe just write bytes?
        <Arr<Var<i32>, Nbt<BlockEntity>>>::proto_encode(&value.block_entities, dst)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UpdateLight {
    pub chunk_x: Var<i32>,
    pub chunk_z: Var<i32>,
    pub sky_light_mask: Var<i32>,
    pub block_light_mask: Var<i32>,
    pub empty_sky_light_mask: Var<i32>,
    pub empty_block_light_mask: Var<i32>,
    pub sky_lights: Vec<u8>,
    pub block_lights: Vec<u8>,
}
impl PacketWrite for UpdateLight {}

impl ProtocolWrite for UpdateLight {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x25.into(), dst)?; // packet_id: 0x25
        <Var<i32>>::proto_encode(&value.chunk_x, dst)?;
        <Var<i32>>::proto_encode(&value.chunk_z, dst)?;
        <Var<i32>>::proto_encode(&value.sky_light_mask, dst)?;
        <Var<i32>>::proto_encode(&value.block_light_mask, dst)?;
        <Var<i32>>::proto_encode(&value.empty_sky_light_mask, dst)?;
        <Var<i32>>::proto_encode(&value.empty_block_light_mask, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.sky_lights, dst)?;
        <Arr<Var<i32>, u8>>::proto_encode(&value.block_lights, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct JoinGame {
    pub entity_id: i32,
    pub game_mode: u8,
    pub dimension: i32,
    pub hashed_seed: u64,
    pub max_players: u8,
    pub level_type: String,
    pub view_distance: Var<i32>,
    pub reduced_debug_info: bool,
    pub enable_respawn_screen: bool,
}
impl PacketWrite for JoinGame {}

impl ProtocolWrite for JoinGame {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x26.into(), dst)?; // packet_id: 0x26
        i32::proto_encode(&value.entity_id, dst)?;
        u8::proto_encode(&value.game_mode, dst)?;
        i32::proto_encode(&value.dimension, dst)?;
        u64::proto_encode(&value.hashed_seed, dst)?;
        u8::proto_encode(&value.max_players, dst)?;
        String::proto_encode(&value.level_type, dst)?;
        <Var<i32>>::proto_encode(&value.view_distance, dst)?;
        bool::proto_encode(&value.reduced_debug_info, dst)?;
        bool::proto_encode(&value.enable_respawn_screen, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub action: PlayerInfoAction,
}
impl PacketWrite for PlayerInfo {}

impl ProtocolWrite for PlayerInfo {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x34.into(), dst)?; // packet_id: 0x34
        PlayerInfoAction::proto_encode(&value.action, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Properties {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    pub signature: Option<String>,
}

impl ProtocolWrite for Properties {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        String::proto_encode(&value.name, dst)?;
        String::proto_encode(&value.value, dst)?;
        bool::proto_encode(&value.is_signed, dst)?;
        Option::proto_encode(&value.signature, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddPlayer {
    pub uuid: Uuid,
    pub name: String,
    pub props: Vec<Properties>,
    pub gamemode: Var<i32>,
    pub ping: Var<i32>,
    pub has_display_name: bool,
    pub display_name: Option<String>,
}

impl ProtocolWrite for AddPlayer {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        Uuid::proto_encode(&value.uuid, dst)?;
        String::proto_encode(&value.name, dst)?;
        <Arr<Var<i32>, Properties>>::proto_encode(&value.props, dst)?;
        <Var<i32>>::proto_encode(&value.gamemode, dst)?;
        <Var<i32>>::proto_encode(&value.ping, dst)?;
        bool::proto_encode(&value.has_display_name, dst)?;
        Option::proto_encode(&value.display_name, dst)?;
        Ok(())
    }
}

impl ProtocolWrite for (Uuid, Var<i32>) {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        Uuid::proto_encode(&value.0, dst)?;
        <Var<i32>>::proto_encode(&value.1, dst)?;
        Ok(())
    }
}

impl ProtocolWrite for (Uuid, bool, Option<String>) {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        Uuid::proto_encode(&value.0, dst)?;
        bool::proto_encode(&value.1, dst)?;
        Option::proto_encode(&value.2, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayerInfoAction {
    AddPlayer(Vec<AddPlayer>),
    UpdateGamemode(Vec<(Uuid, Var<i32>)>),
    UpdateLatency(Vec<(Uuid, Var<i32>)>),
    UpdateDisplayName(Vec<(Uuid, bool, Option<String>)>),
    RemovePlayer(Vec<Uuid>),
}

impl ProtocolWrite for PlayerInfoAction {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        match value {
            PlayerInfoAction::AddPlayer(add_player) => {
                <Var<i32>>::proto_encode(&0.into(), dst)?;
                <Arr<Var<i32>, _>>::proto_encode(add_player, dst)?;
            }
            PlayerInfoAction::UpdateGamemode(update_gamemode) => {
                <Var<i32>>::proto_encode(&1.into(), dst)?;
                <Arr<Var<i32>, _>>::proto_encode(update_gamemode, dst)?;
            }
            PlayerInfoAction::UpdateLatency(update_latency) => {
                <Var<i32>>::proto_encode(&2.into(), dst)?;
                <Arr<Var<i32>, _>>::proto_encode(update_latency, dst)?;
            }
            PlayerInfoAction::UpdateDisplayName(update_display_name) => {
                <Var<i32>>::proto_encode(&3.into(), dst)?;
                <Arr<Var<i32>, _>>::proto_encode(update_display_name, dst)?;
            }
            PlayerInfoAction::RemovePlayer(remove_player) => {
                <Var<i32>>::proto_encode(&4.into(), dst)?;
                <Arr<Var<i32>, _>>::proto_encode(remove_player, dst)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct PlayerPositionAndLook {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: u8,
    pub teleport_id: Var<i32>,
}
impl PacketWrite for PlayerPositionAndLook {}

impl ProtocolWrite for PlayerPositionAndLook {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x36.into(), dst)?; // packet_id: 0x36
        f64::proto_encode(&value.x, dst)?;
        f64::proto_encode(&value.y, dst)?;
        f64::proto_encode(&value.z, dst)?;
        f32::proto_encode(&value.yaw, dst)?;
        f32::proto_encode(&value.pitch, dst)?;
        u8::proto_encode(&value.flags, dst)?;
        <Var<i32>>::proto_encode(&value.teleport_id, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UnlockRecipes {
    pub action: Var<i32>,
    pub crafting_recipe_book_open: bool,
    pub crafting_recipe_book_filter_active: bool,
    pub smelting_recipe_book_open: bool,
    pub smelting_recipe_book_filter_active: bool,
    pub recipe_ids: Vec<Var<i32>>,
    pub additional_recipe_ids: Option<Vec<Var<i32>>>,
}
impl PacketWrite for UnlockRecipes {}

impl ProtocolWrite for UnlockRecipes {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x37.into(), dst)?; // packet_id: 0x37
        <Var<i32>>::proto_encode(&value.action, dst)?;
        bool::proto_encode(&value.crafting_recipe_book_open, dst)?;
        bool::proto_encode(&value.crafting_recipe_book_filter_active, dst)?;
        bool::proto_encode(&value.smelting_recipe_book_open, dst)?;
        bool::proto_encode(&value.smelting_recipe_book_filter_active, dst)?;
        <Arr<Var<i32>, Var<i32>>>::proto_encode(&value.recipe_ids, dst)?;
        // TODO: Optional Array
        match &value.additional_recipe_ids {
            Some(v) => <Arr<Var<i32>, Var<i32>>>::proto_encode(v, dst)?,
            None => {}
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WorldBorder {
    pub action: WorldBorderAction,
}
impl PacketWrite for WorldBorder {}

impl ProtocolWrite for WorldBorder {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x3E.into(), dst)?; // packet_id: 0x3E
        WorldBorderAction::proto_encode(&value.action, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum WorldBorderAction {
    SetSize {
        diameter: f64,
    },
    LerpSize {
        old_diameter: f64,
        new_diameter: f64,
        speed: Var<i64>,
    },
    SetCenter {
        x: f64,
        z: f64,
    },
    Initialize {
        x: f64,
        z: f64,
        old_diameter: f64,
        new_diameter: f64,
        speed: Var<i64>,
        portal_teleport_boundary: Var<i32>,
        warning_time: Var<i32>,
        warning_blocks: Var<i32>,
    },
    SetWarningTime {
        warning_time: Var<i32>,
    },
    SetWarningBlocks {
        warning_blocks: Var<i32>,
    },
}

impl ProtocolWrite for WorldBorderAction {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        match value {
            WorldBorderAction::SetSize { diameter } => {
                <Var<i32>>::proto_encode(&0.into(), dst)?;
                f64::proto_encode(diameter, dst)?;
            }
            WorldBorderAction::LerpSize {
                old_diameter,
                new_diameter,
                speed,
            } => {
                <Var<i32>>::proto_encode(&1.into(), dst)?;
                f64::proto_encode(old_diameter, dst)?;
                f64::proto_encode(new_diameter, dst)?;
                <Var<i64>>::proto_encode(speed, dst)?;
            }
            WorldBorderAction::SetCenter { x, z } => {
                <Var<i32>>::proto_encode(&2.into(), dst)?;
                f64::proto_encode(x, dst)?;
                f64::proto_encode(z, dst)?;
            }
            WorldBorderAction::Initialize {
                x,
                z,
                old_diameter,
                new_diameter,
                speed,
                portal_teleport_boundary,
                warning_time,
                warning_blocks,
            } => {
                <Var<i32>>::proto_encode(&3.into(), dst)?;
                f64::proto_encode(x, dst)?;
                f64::proto_encode(z, dst)?;
                f64::proto_encode(old_diameter, dst)?;
                f64::proto_encode(new_diameter, dst)?;
                <Var<i64>>::proto_encode(speed, dst)?;
                <Var<i32>>::proto_encode(portal_teleport_boundary, dst)?;
                <Var<i32>>::proto_encode(warning_time, dst)?;
                <Var<i32>>::proto_encode(warning_blocks, dst)?;
            }
            WorldBorderAction::SetWarningTime { warning_time } => {
                <Var<i32>>::proto_encode(&4.into(), dst)?;
                <Var<i32>>::proto_encode(warning_time, dst)?;
            }
            WorldBorderAction::SetWarningBlocks { warning_blocks } => {
                <Var<i32>>::proto_encode(&5.into(), dst)?;
                <Var<i32>>::proto_encode(warning_blocks, dst)?;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HeldItemChange {
    pub slot: u8,
}
impl PacketWrite for HeldItemChange {}

impl ProtocolWrite for HeldItemChange {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x40.into(), dst)?; // packet_id: 0x40
        u8::proto_encode(&value.slot, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UpdateViewPosition {
    pub chunk_x: Var<i32>,
    pub chunk_z: Var<i32>,
}
impl PacketWrite for UpdateViewPosition {}

impl ProtocolWrite for UpdateViewPosition {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x41.into(), dst)?; // packet_id: 0x41
        <Var<i32>>::proto_encode(&value.chunk_x, dst)?;
        <Var<i32>>::proto_encode(&value.chunk_z, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SpawnPosition {
    pub location: Position,
}
impl PacketWrite for SpawnPosition {}

impl ProtocolWrite for SpawnPosition {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x4E.into(), dst)?; // packet_id: 0x4E
        Position::proto_encode(&value.location, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct DeclareRecipes {
    pub recipes: Vec<Recipe>,
}
impl PacketWrite for DeclareRecipes {}

impl ProtocolWrite for DeclareRecipes {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x5B.into(), dst)?; // packet_id: 0x5B
        <Arr<Var<i32>, Recipe>>::proto_encode(&value.recipes, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Recipe {
    recipe_type: String,
    recipe_id: String,
}

impl ProtocolWrite for Recipe {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        String::proto_encode(&value.recipe_type, dst)?;
        String::proto_encode(&value.recipe_id, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Tags {
    pub block_tags: Vec<Tag>,
    pub item_tags: Vec<Tag>,
    pub fluid_tags: Vec<Tag>,
    pub entity_tags: Vec<Tag>,
}
impl PacketWrite for Tags {}

impl ProtocolWrite for Tags {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        <Var<i32>>::proto_encode(&0x5C.into(), dst)?; // packet_id: 0x5C
        <Arr<Var<i32>, Tag>>::proto_encode(&value.block_tags, dst)?;
        <Arr<Var<i32>, Tag>>::proto_encode(&value.item_tags, dst)?;
        <Arr<Var<i32>, Tag>>::proto_encode(&value.fluid_tags, dst)?;
        <Arr<Var<i32>, Tag>>::proto_encode(&value.entity_tags, dst)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    name: String,
    entries: Vec<Var<i32>>,
}

impl ProtocolWrite for Tag {
    fn proto_encode(value: &Self, dst: &mut dyn Write) -> io::Result<()> {
        String::proto_encode(&value.name, dst)?;
        <Arr<Var<i32>, Var<i32>>>::proto_encode(&value.entries, dst)?;
        Ok(())
    }
}
