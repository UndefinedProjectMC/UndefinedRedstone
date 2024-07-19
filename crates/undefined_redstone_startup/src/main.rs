use bevy_app::{App, PostStartup, ScheduleRunnerPlugin};
use bevy_ecs::prelude::IntoSystemConfigs;
use undefined_redstone_asyncmanager::URAsyncManagerPlugin;
use undefined_redstone_core::startup::{finish_startup, URStartupPlugin};
use undefined_redstone_network::URNetworkPlugin;
use undefined_redstone_packloader::URPackLoaderPlugin;
use undefined_redstone_protocol::URProtocolPlugin;

fn main() {
    App::new()
        .add_plugins(ScheduleRunnerPlugin::run_loop(std::time::Duration::from_secs_f64(1.0 / 20.0)))
        .add_plugins(URStartupPlugin)
        .add_plugins(URAsyncManagerPlugin)
        .add_plugins(URPackLoaderPlugin)
        .add_plugins(URNetworkPlugin)
        .add_plugins(URProtocolPlugin)
        .add_systems(PostStartup, finish_startup)
        //.add_systems(PostStartup, command.after(finish_startup))
        .run();
}

fn command() {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}