use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use arc_swap::ArcSwap;
use bevy_ecs::entity::Entity;
use bevy_ecs::world::{Mut, World};
use uuid::Uuid;
use bevy_async_ecs::AsyncWorld;
use rak_rs::connection::Connection;
use rak_rs::Listener;
use undefined_redstone_asyncmanager::URAsyncManager;
use undefined_redstone_core::server::Server;
use undefined_redstone_core::utils::server_properties::ServerProperties;
use undefined_redstone_log::t;
use undefined_redstone_nbt::NbtValue;
use undefined_redstone_packloader::entity::EntityContentManager;
use undefined_redstone_type::display_name::DisplayName;
use crate::client::MinecraftClient;
use crate::ip::IPService;
use crate::packet::packet_factory::PacketFactory;
use crate::packet::packet_handler::{ServerPacketHandler, ServerPacketHandlerStatus};
use crate::URNetworkSettings;

static mut LISTENER: Option<Listener> = None;

pub(crate) async fn accept_connection(settings: URNetworkSettings) {
    unsafe {
        let mut listener = Listener::bind(format!("0.0.0.0:{}", settings.server_port)).await.unwrap();
        listener.id = settings.guid;
        listener.motd = Arc::new(ArcSwap::from(Arc::new(settings.server_motd.clone())));
        listener.start().await.unwrap();
        LISTENER = Some(listener);
        loop {
            if let Ok(connection) = LISTENER.as_mut().unwrap().accept().await {
                tokio::spawn(handle_connection(connection, settings.clone()));
            }
        }
    }
}

pub(crate) async fn handle_connection(connection: Connection, settings: URNetworkSettings) {
    let mut packet_factory = PacketFactory::new(connection, settings.clone());
    loop {
        let packets = packet_factory.recv_packet().await;
        if let Ok(status) = ServerPacketHandler::handle_packet(&mut packet_factory, packets).await {
            match status {
                ServerPacketHandlerStatus::Closed => {
                    println!("Closed");
                    return;
                }
                ServerPacketHandlerStatus::CreatePlayer => {
                    if let Ok(_) = settings.inner.sender.send(packet_factory) {
                        println!("new client");
                    }
                    return;
                }
                _ => {
                    continue;
                }
            }
        }
    }
}

pub(crate) async fn new_client(world: AsyncWorld) {
    let settings = world.wait_for_resource::<URNetworkSettings>().await;
    loop {
        for _ in 0..settings.inner.receiver.len() {
            tokio::spawn(create_new_client(world.clone(), settings.clone()));
        }
    }
}
pub(crate) async fn create_new_client(world: AsyncWorld, settings: URNetworkSettings) {
    if let Ok(factory) = settings.inner.receiver.try_recv() {
        let username = factory.data.username.clone();
        let ip = factory.connection.address.ip().to_string();
        let ip_service = IPService::new(ip.clone()).await;
        let locate = ip_service.locate;
        let os_list = vec!["Unknown", "Android", "iOS", "macOS", "FireOS", "GearVR", "HoloLens", "Windows", "Windows", "Education", "Dedicated", "PlayStation", "Switch", "Xbox"];
        let os = os_list.get(factory.data.device_os as usize).cloned().unwrap_or("Unknown");
        let device = factory.data.device_model.clone();
        let xuid = factory.data.uuid.to_string();
        println!("{}", t!("console.player.join", player = username, ip = ip, ip_locate = locate, os = os, device = device, xuid = xuid));
        spawn_client(world, settings, factory).await;
    }
}

pub(crate) async fn spawn_client(world: AsyncWorld,
                                 settings: URNetworkSettings,
                                 factory: PacketFactory) {
    unsafe { LISTENER.as_ref().unwrap().update_player_online(1); } //更新motd
    let entity_manager = world.wait_for_resource::<EntityContentManager>().await;
    let entity = entity_manager.get("minecraft:player").unwrap();
    let username = factory.data.username.clone();
    let usernamex = username.clone();
    let xuid = factory.data.uuid.to_string();
    let xuid = Uuid::from_str(xuid.as_str()).unwrap();
    world.apply(move |mut world: &mut World| {
        let data = factory.data.to_client_data();
        let connection = factory.into_client_connection();
        let id;
        {
            let mut client = world.spawn(MinecraftClient::new(
                connection,
                data
            ));
            id = client.id();
            entity.insert(&mut client);
            client.insert(DisplayName(usernamex.clone()));
        }
        println!("process login");
        process_login(world, xuid, usernamex.clone(), id);
    }).await;
    println!("{}", t!("console.player.spawn", player = username, xuid = xuid));
}

//史山代码等待重构...
fn process_login(world: &mut World, xuid: Uuid, display_name: String, entity_id: Entity) {
    let default_gamemode;
    let force_gamemode;
    {
        let server_properties = world.resource_ref::<ServerProperties>();
        default_gamemode = server_properties.game_mode.clone();
        force_gamemode = server_properties.force_gamemode;
    }

    let async_manager = URAsyncManager::global();

    // 提前获取需要的不可变数据
    let online_clients = {
        let server = world.resource_ref::<Server>();
        server.get_online_clients().clone()
    };

    let is_online_player = online_clients.iter().find(|(uuid, entity)| {
        if let Some(name) = world.get::<DisplayName>(**entity) {
            name.0 == display_name || uuid == &&xuid
        } else {
            false
        }
    }).clone();

    if let Some((_, entity)) = is_online_player {
        // 获取可变引用并处理已在线玩家逻辑
        let mut client = world.get_mut::<MinecraftClient>(*entity).unwrap();
        let _ = async_manager.runtime("network").unwrap().block_on(
            client.disconnect("disconnectionScreen.loggedinOtherLocation", false)
        );
    } else {
        // 获取可变引用并处理新的登录逻辑
        let mut data = world.get::<MinecraftClient>(entity_id).unwrap().data.clone();
        let mut server = world.resource_mut::<Server>();
        data.is_first_play = server.get_client(xuid.clone()).is_none();
        let nbt = server.get_offline_player_nbt(xuid, default_gamemode.clone());

        if let Some(mut nbt) = nbt {
            //更新db
            let last_played = nbt.get("lastPlayed").unwrap().as_i64().unwrap();
            let first_played = nbt.get("firstPlayed").unwrap().as_i64().unwrap();
            data.play_before_time = last_played - first_played;

            nbt.insert("NameTag", NbtValue::String(display_name.clone()));

            let exp = nbt.get("EXP").unwrap().as_i32().unwrap();
            let exp_level = nbt.get("expLevel").unwrap().as_i32().unwrap();
            data.set_experience(exp, exp_level);

            if force_gamemode {
                data.gamemode = default_gamemode;
                nbt.insert("playerGameType", NbtValue::Int(data.gamemode.to_i32()));
            }

            //level处理
            /*
            Level level;
            if ((level = this.server.getLevelByName(nbt.getString("Level"))) == null) {
                this.setLevel(this.server.getDefaultLevel());
                nbt.putString("Level", this.level.getName());
                Position spawnLocation = this.level.getSafeSpawn();
                nbt.getList("Pos", DoubleTag.class)
                        .add(new DoubleTag(spawnLocation.x))
                        .add(new DoubleTag(spawnLocation.y))
                        .add(new DoubleTag(spawnLocation.z));
            } else {
                this.setLevel(level);
            }
             */

            let achievements = nbt.get("Achievements").unwrap().as_compound().unwrap();
            for (name, value) in achievements.iter() {
                if let Some(value) = value.as_i16() {
                    if value > 0 {
                        data.achievements.push(name.to_string())
                    }
                }
            }

            data.first_played = nbt.get("firstPlayed").unwrap().as_i64().unwrap();
            data.before_login_last_played = nbt.get("lastPlayed").unwrap().as_i64().unwrap();

            let now_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
            nbt.insert("lastPlayed", NbtValue::Long(now_time));

            let uuid = xuid.as_u64_pair();
            nbt.insert("UUIDLeast", NbtValue::Long(uuid.1 as i64));
            nbt.insert("UUIDMost", NbtValue::Long(uuid.0 as i64));

            //auto save

            println!("finish nbt");
            {
                let mut client = world.get_mut::<MinecraftClient>(entity_id).unwrap();
                client.data = data;
                client_start_game(client);
            }
        } else {
            let mut client = world.get_mut::<MinecraftClient>(entity_id).unwrap();
            let _ = async_manager.runtime("network").unwrap().block_on(
                client.disconnect("Invalid data", false)
            );
        }
    }
}

fn client_start_game(client: Mut<MinecraftClient>) {

}

