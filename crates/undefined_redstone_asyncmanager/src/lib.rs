use std::collections::HashMap;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::prelude::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Resource};
use tokio::runtime::{EnterGuard, Runtime};
use undefined_redstone_core::startup::URPreStartupSet;

#[derive(Resource)]
pub struct URAsyncManager(HashMap<String, URAsyncManagerInner>);

impl URAsyncManager {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: &str) {
        self.0.insert(key.to_string(), URAsyncManagerInner::new());
    }

    pub fn enter(&self, key: &str) -> Option<EnterGuard<'_>> {
        Some(self.0.get(key)?.runtime.enter())
    }
}

pub(crate) struct URAsyncManagerInner {
    runtime: Runtime
}

impl URAsyncManagerInner {
    pub fn new() -> Self {
        Self {
            runtime: Runtime::new().unwrap(),
        }
    }
}

pub struct URAsyncManagerPlugin;

fn init_async_manager(mut commands: Commands) {
    commands.insert_resource(URAsyncManager::new());
}

impl Plugin for URAsyncManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, init_async_manager.in_set(URPreStartupSet));
    }
}