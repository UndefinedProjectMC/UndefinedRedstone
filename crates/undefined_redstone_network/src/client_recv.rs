use std::time::{SystemTime, UNIX_EPOCH};
use bevy_ecs::component::Component;
use bevy_ecs::entity::Entity;
use bevy_ecs::event::Event;
use bevy_ecs::world::{FromWorld, World};
use tokio::task::JoinHandle;
use bevy_async_ecs::AsyncWorld;
use undefined_redstone_asyncmanager::URAsyncManager;
use crate::client::MinecraftClient;
use crate::packet::batch_packet::OriginBatchPacket;

#[derive(Event, Clone, Debug)]
pub struct ClientConnectionRecvEvent {
    pub client_id: Entity,
    pub timestamp: u128,
    pub packet: OriginBatchPacket
}

#[derive(Component)]
pub struct ClientConnectionRecvThread {
    pub thread: JoinHandle<()>
}

pub fn recv_event_loop(world: &mut World) {
    let mut entities = vec![];
    {
        let mut query = world.query::<(Entity, &MinecraftClient)>();
        for (entity, _) in query.iter(world) {
            entities.push(entity)
        }
    }
    for entity in entities {
        if let None = world.get::<ClientConnectionRecvThread>(entity) {
            let runtime = URAsyncManager::global().runtime("network").unwrap();
            let client_id = entity.clone();
            let async_world = { AsyncWorld::from_world(world) };
            let thread = runtime.spawn(async move {
                async_world.apply(move |world: &mut World| {
                    loop {
                        let mut client = world.get_mut::<MinecraftClient>(client_id).unwrap();
                        let p = URAsyncManager::global().runtime("network").unwrap().block_on(
                            client.connection.recv_packet()
                        );
                        if let Ok(ref packet) = p {
                            let timestamp = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis();
                            world.send_event(ClientConnectionRecvEvent {
                                client_id,
                                timestamp,
                                packet: packet.clone(),
                            });
                        }
                    }
                }).await;
            });
            println!("spawn thread");
            world.entity_mut(entity).insert(ClientConnectionRecvThread {
                thread
            });
        }
    }
}