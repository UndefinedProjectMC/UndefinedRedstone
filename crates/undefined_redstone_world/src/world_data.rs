use serde::Deserialize;
use serde_inline_default::serde_inline_default;
use undefined_redstone_type::client_abilities::ClientAbilities;
use undefined_redstone_type::difficulty::Difficulty;
use undefined_redstone_type::experiment::ExperimentData;
use undefined_redstone_type::gamemode::Gamemode;
use undefined_redstone_type::gamerules::Gamerules;
use crate::world_layers::WorldLayers;
use crate::world_policies::WorldPolicies;
use undefined_redstone_nbt::deserialize_nbt_bool;

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct MinecraftWorldData {
    #[serde_inline_default(Difficulty::Easy)]
    #[serde(alias="Difficulty")]
    pub difficulty: Difficulty,
    #[serde_inline_default(false)]
    #[serde(alias="ForceGameType")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub force_gamemode: bool,
    #[serde_inline_default(Gamemode::Survival)]
    #[serde(alias="GameType")]
    pub gamemode: Gamemode,
    #[serde_inline_default("UndefinedRedstone Level".to_string())]
    #[serde(alias="LevelName")]
    pub world_name: String,
    #[serde(alias="SpawnX")]
    pub spawn_x: i32,
    #[serde(alias="SpawnY")]
    pub spawn_y: i32,
    #[serde(alias="SpawnZ")]
    pub spawn_z: i32,
    #[serde(alias="Time")]
    pub time: i64,
    #[serde(alias="GameRules")]
    #[serde(skip_deserializing)]//因为没有实现读取nbt
    pub gamerules: Gamerules,
    #[serde(alias="LimitedWorldOriginX")]
    pub limited_world_origin_x: i32,
    #[serde(alias="LimitedWorldOriginY")]
    pub limited_world_origin_y: i32,
    #[serde(alias="LimitedWorldOriginZ")]
    pub limited_world_origin_z: i32,
    #[serde(alias="MinimumCompatibleClientVersion")]
    #[serde(skip_deserializing)]//因为没有实现读取array
    pub minimum_compatible_minecraft_version: String,
    #[serde(alias="LANBroadcast")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub lan_broadcast: bool,
    #[serde(alias="LANBroadcastIntent")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub lan_broadcast_intent: bool,
    #[serde(alias="LastPlayed")]
    pub last_played: i64,
    #[serde(alias="BiomeOverride")]
    pub biome_override: String,
    #[serde(alias="CenterMapsToOrigin")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub center_maps_to_origin: bool,
    #[serde(alias="ConfirmedPlatformLockedContent")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub confirmed_platform_locked_content: bool,
    #[serde(alias="FlatWorldLayers")]
    #[serde(skip_deserializing)]//因为没有实现读取nbt
    pub flat_world_layers: WorldLayers,
    #[serde(alias="Generator")]
    pub generator: i32,
    #[serde(alias="InventoryVersion")]
    pub inventory_version: String,
    #[serde(alias="MultiplayerGame")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub multiplayer_game: bool,
    #[serde(alias="MultiplayerGameIntent")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub multiplayer_game_intent: bool,
    #[serde(alias="NetherScale")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub nether_scale: bool,
    #[serde(alias="NetworkVersion")]
    pub protocol_version: i32,
    #[serde(alias="Platform")]
    pub platform: i32,
    #[serde(alias="PlatformBroadcastIntent")]
    pub platform_broadcast_intent: i32,
    #[serde(alias="RandomSeed")]
    pub world_seed: i64,
    #[serde(alias="SpawnV1Villagers")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub spawn_v1_villagers: bool,
    #[serde(alias="StorageVersion")]
    pub storage_version: i32,
    #[serde(alias="WorldVersion")]
    pub world_version: i32,
    #[serde(alias="XBLBroadcastIntent")]
    pub xbl_broadcast_intent: i32,
    #[serde(alias="abilities")]
    #[serde(skip_deserializing)]//因为没有实现读取nbt
    pub client_abilities: ClientAbilities,
    #[serde(alias="baseGameVersion")]
    #[serde_inline_default(vec![])]
    pub base_game_version: Vec<u8>,
    #[serde(alias="bonusChestEnabled")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub bonus_chest_enabled: bool,
    #[serde(alias="bonusChestSpawned")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub bonus_chest_spawned: bool,
    #[serde(alias="cheatsEnabled")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub cheats_enabled: bool,
    #[serde(alias="commandsEnabled")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub commands_enabled: bool,
    #[serde(alias="currentTick")]
    pub current_tick: i64,
    #[serde(alias="daylightCycle")]
    pub daylight_cycle: i32,
    #[serde(alias="editorWorldType")]
    pub editor_world_type: i32,
    #[serde(alias="eduOffer")]
    pub edu_offer: i32,
    #[serde(alias="educationFeaturesEnabled")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub edu_features_enabled: bool,
    #[serde(alias="experiments")]
    #[serde(skip_deserializing)]//因为没有实现读取nbt
    pub experiments: Vec<ExperimentData>,
    #[serde(alias="hasBeenLoadedInCreative")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub has_been_loaded_in_creative: bool,
    #[serde(alias="hasLockedBehaviorPack")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub has_locked_behavior_pack: bool,
    #[serde(alias="hasLockedResourcePack")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub has_locked_resource_pack: bool,
    #[serde(alias="immutableWorld")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub immutable_world: bool,
    #[serde(alias="isCreatedInEditor")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_created_in_editor: bool,
    #[serde(alias="isExportedFromEditor")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_exported_from_editor: bool,
    #[serde(alias="isFromLockedTemplate")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_from_locked_template: bool,
    #[serde(alias="isFromWorldTemplate")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_from_world_template: bool,
    #[serde(alias="isRandomSeedAllowed")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_random_seed_allowed: bool,
    #[serde(alias="isSingleUseWorld")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_single_use_world: bool,
    #[serde(alias="isWorldTemplateOptionLocked")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub is_world_template_option_locked: bool,
    #[serde(alias="lastOpenedWithVersion")]
    pub last_opened_with_version: Vec<u8>,
    #[serde(alias="lightningLevel")]
    pub lightning_level: f32,
    #[serde(alias="lightningTime")]
    pub lightning_time: i32,
    #[serde(alias="limitedWorldDepth")]
    pub limited_world_depth: i32,
    #[serde(alias="limitedWorldWidth")]
    pub limited_world_width: i32,
    #[serde(alias="permissionsLevel")]
    pub permissions_level: i32,
    #[serde(alias="playerPermissionsLevel")]
    pub player_permissions_level: i32,
    #[serde(alias="playerSleepingPercentage")]
    #[serde_inline_default(0)]
    pub player_sleeping_percentage: i32,
    #[serde(alias="prid")]
    pub prid: String,
    #[serde(alias="rainLevel")]
    pub rain_level: f32,
    #[serde(alias="rainTime")]
    pub rain_time: i32,
    #[serde(alias="randomTickSpeed")]
    #[serde_inline_default(0)]
    pub random_tick_speed: i32,
    #[serde(alias="recipesunlock")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub recipes_unlock: bool,
    #[serde(alias="requiresCopiedPackRemovalCheck")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub requires_copied_pack_removal_check: bool,
    #[serde(alias="serverChunkTickRange")]
    pub server_chunk_tick_range: i32,
    #[serde(alias="spawnMobs")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub spawn_mobs: bool,
    #[serde(alias="startWithMapEnabled")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub start_with_map_enabled: bool,
    #[serde(alias="texturePacksRequired")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    pub texture_packs_required: bool,
    #[serde(alias="useMsaGamerTagsOnly")]
    #[serde(deserialize_with = "deserialize_nbt_bool")]
    #[serde_inline_default(false)]
    pub use_msa_gamer_tags_only: bool,
    #[serde(alias="worldStartCount")]
    pub world_start_count: i64,
    #[serde_inline_default(WorldPolicies::new())]
    pub world_policies: WorldPolicies,
}

impl Default for MinecraftWorldData {
    fn default() -> Self {
        todo!()
    }
}

impl MinecraftWorldData {
    pub const WORLD_DATA_MAGIC_NUMBER: [u8; 8] = [10, 0, 0, 0, 68, 11, 0, 0];
}