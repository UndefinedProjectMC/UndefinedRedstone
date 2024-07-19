use std::collections::HashMap;
use bevy_app::{App, Plugin, PreStartup};
use bevy_ecs::prelude::IntoSystemConfigs;
use bevy_ecs::system::{Commands, Resource};
use tokio::runtime::{EnterGuard, Runtime};
use undefined_redstone_core::startup::URPreStartupSet;

static mut ASYNC_MANAGER: Option<URAsyncManager> = None;

pub struct URAsyncManager(HashMap<String, URAsyncManagerInner>);

impl URAsyncManager {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn global() -> &'static Self {
        unsafe { ASYNC_MANAGER.as_ref().unwrap() }
    }

    pub fn insert(&self, key: &str) {
        unsafe { ASYNC_MANAGER.as_mut().unwrap().0.insert(key.to_string(), URAsyncManagerInner::new()) };
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

impl Plugin for URAsyncManagerPlugin {
    fn build(&self, app: &mut App) {
        unsafe { ASYNC_MANAGER = Some(URAsyncManager::new()) };
    }
}