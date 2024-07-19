use bevy_ecs::component::Component;
use crate::client_connection::ClientConnection;

#[derive(Component)]
pub struct MinecraftClient {
    pub connection: ClientConnection
}