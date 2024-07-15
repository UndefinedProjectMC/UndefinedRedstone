use bevy_app::{App, PostStartup};
use bevy_ecs::prelude::{IntoSystemConfigs, Resource};
use rust_i18n::i18n;
use undefined_redstone_network::URNetworkPlugin;
use crate::startup::URStartupPlugin;

pub mod entities;
pub mod utils;
pub mod server;
pub mod startup;
pub mod commands;
pub mod blocks;

i18n!("locales");

fn main() {
    App::new()
        .add_plugins(URStartupPlugin)
        .add_plugins(URNetworkPlugin)
        .add_systems(PostStartup, command)
        .run();
}

fn command() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}