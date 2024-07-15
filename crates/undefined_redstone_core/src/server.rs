use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct Server {
    pub server_version: String
}

impl Default for Server {
    fn default() -> Self {
        Self {
            server_version: "1.0.0ALPHA(fjord)".to_string(),
        }
    }
}