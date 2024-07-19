use bevy_ecs::component::Component;

#[derive(Component, Debug, Clone)]
pub struct DisplayName(pub String);