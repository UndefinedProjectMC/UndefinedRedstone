use std::borrow::Borrow;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use bevy_ecs::system::Resource;
use toml::Table;
use rand::random;
use undefined_redstone_type::gamemode::Gamemode;
use rak_rs::Motd;

#[derive(Clone, Debug, Resource)]
pub struct ServerProperties {
    pub enable_snappy: bool,
    pub server_name: String,
    pub server_motd: String,
    pub max_player: u32,
    pub game_mode: Gamemode,
    pub ipv4_port: u16,
    pub ipv6_port: u16,
    pub guid: u64,
    pub xbox_auth: bool,
    pub validate_encryption: bool,
    pub force_gamemode: bool,
    pub experimental_data_driven_items: bool,
    pub experimental_data_driven_biomes: bool,
    pub experimental_upcoming_creator_features: bool,
    pub experimental_gametest: bool,
    pub experimental_molang_features: bool,
    pub experimental_cameras: bool,
}

impl ServerProperties {
    pub fn load<P: AsRef<Path>>(file_path: P) -> Option<Self> {
        let mut file = File::open(file_path).ok()?;
        let mut file_text = String::new();
        file.read_to_string(&mut file_text).ok()?;
        let toml = file_text.parse::<Table>().ok()?;
        Self::parse(toml)
    }

    fn parse(toml: Table) -> Option<Self> {
        //game
        let game = toml.get("game")?;
        let server_name = game.get("name")?.as_str()?;
        let server_motd = game.get("motd")?.as_str()?;
        let max_player = game.get("max_player")?.as_integer()? as u32;
        let game_mode = game.get("gamemode")?.as_str()?.to_ascii_lowercase();
        let game_mode = game_mode.as_str();
        let game_mode = if game_mode == "creative" || game_mode == "c" {
            Gamemode::Creative
        }else if game_mode == "adventure" || game_mode == "a" {
            Gamemode::Adventure
        }else if game_mode == "spectator" || game_mode == "s" {
            Gamemode::Spectator
        }else {
            Gamemode::Survival
        };
        let ipv4_port = game.get("ipv4_port")?.as_integer()? as u16;
        let ipv6_port = game.get("ipv6_port")?.as_integer()? as u16;

        //server
        let server = toml.get("server")?;
        let enable_snappy = server.get("enable_snappy")?.as_bool()?;
        let xbox_auth = server.get("xbox_auth")?.as_bool()?;
        let validate_encryption = server.get("validate_encryption")?.as_bool()?;
        let force_gamemode = server.get("force_gamemode")?.as_bool()?;

        //experimental_gameplay
        let experimental_gameplay = toml.get("experimental_gameplay")?;
        let experimental_data_driven_items = experimental_gameplay.get("data_driven_items")?.as_bool()?;
        let experimental_data_driven_biomes = experimental_gameplay.get("data_driven_biomes")?.as_bool()?;
        let experimental_upcoming_creator_features = experimental_gameplay.get("upcoming_creator_features")?.as_bool()?;
        let experimental_gametest = experimental_gameplay.get("gametest")?.as_bool()?;
        let experimental_molang_features = experimental_gameplay.get("experimental_molang_features")?.as_bool()?;
        let experimental_cameras = experimental_gameplay.get("cameras")?.as_bool()?;

        Some(Self {
            enable_snappy,
            server_name: server_name.to_string(),
            server_motd: server_motd.to_string(),
            max_player,
            game_mode,
            ipv4_port,
            ipv6_port,
            guid: random(),
            xbox_auth,
            validate_encryption,
            force_gamemode,
            experimental_data_driven_items,
            experimental_data_driven_biomes,
            experimental_upcoming_creator_features,
            experimental_gametest,
            experimental_molang_features,
            experimental_cameras,
        })
    }

    pub fn to_motd(&self) -> Motd {
        Motd {
            motd: self.server_motd.clone(),
            protocol: 685,
            version: "1.21.0".to_string(),
            player_online: 0,
            player_max: self.max_player,
            gamemode: self.game_mode,
            server_guid: self.guid,
            level_name: "UndefinedRedstore Server".to_string(),
        }
    }
}