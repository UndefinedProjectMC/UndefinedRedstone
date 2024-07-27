use bevy_ecs::component::Component;
use uuid::Uuid;
use undefined_redstone_type::gamemode::Gamemode;
use crate::client_connection::ClientConnection;

#[derive(Clone)]
pub struct MinecraftClientData {
    pub display_name: String,
    pub uuid: Uuid,
    pub play_before_time: i64,
    pub gamemode: Gamemode,
    pub achievements: Vec<String>,
    pub is_first_play: bool,
    pub first_played: i64,
    pub before_login_last_played: i64,
}

impl Default for MinecraftClientData {
    fn default() -> Self {
        Self {
            display_name: String::new(),
            uuid: Uuid::nil(),
            play_before_time: 0,
            gamemode: Gamemode::Survival,
            achievements: vec![],
            is_first_play: false,
            first_played: 0,
            before_login_last_played: 0,
        }
    }
}

impl MinecraftClientData {
    pub fn set_experience(&mut self, exp: i32, exp_level: i32) {

    }
}

#[derive(Component)]
pub struct MinecraftClient {
    pub connection: ClientConnection,
    pub data: MinecraftClientData,
}

impl MinecraftClient {
    pub fn new(connection: ClientConnection, data: MinecraftClientData) -> Self {
        MinecraftClient {
            connection,
            data,
        }
    }

    pub async fn disconnect(&mut self, reason: &str, hide_disconnect_screen: bool) -> anyhow::Result<()> {
        self.connection.disconnect(reason, hide_disconnect_screen).await
    }
}