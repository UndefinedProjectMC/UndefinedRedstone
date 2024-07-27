use undefined_redstone_type::experiment::ExperimentData;
use crate::encryption::MinecraftEncryption;
use crate::packet::batch_packet::BatchPacket;
use crate::packet::packet_factory::{PacketFactory, PacketFactoryStatus};
use crate::protocol::{MinecraftPacket, ProtocolInfo};
use crate::protocol::client::resource_packs::ResourcePackClientResponse;
use crate::protocol::server::handshake::{NetworkSettings, ServerToClientHandshake};
use crate::protocol::server::PlayStatus;
use crate::protocol::server::resource_packs::{ResourcePackInfo, ResourcePackStack};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ServerPacketHandlerStatus {
    Receiving,
    Closed,
    CreatePlayer,
}

pub struct ServerPacketHandler;

impl ServerPacketHandler {

    pub async fn handle_packet(factory: &mut PacketFactory, packets: anyhow::Result<BatchPacket>) -> anyhow::Result<ServerPacketHandlerStatus> {
        let packets = packets?;
        let mut result = Err(anyhow::Error::msg("no packets handle"));
        for packet in packets {
            result = Self::handle_packet0(factory, packet).await;
        }
        result
    }

    async fn handle_packet0(factory: &mut PacketFactory, packet: MinecraftPacket) -> anyhow::Result<ServerPacketHandlerStatus> {
        match packet {
            MinecraftPacket::RequestNetworkSettings(request) => {
                factory.status = PacketFactoryStatus::Logging;
                let protocol_version = request.protocol_version;
                println!("Protocol Version: {}", protocol_version);
                factory.protocol_version = protocol_version;
                let compression_algorithm = factory.settings.compression_algorithm;
                factory.send_packet(MinecraftPacket::NetworkSettings(
                    NetworkSettings {
                        compression_threshold: 1,
                        compression_algorithm,
                        client_throttle_enabled: false,
                        client_throttle_threshold: 0,
                        client_throttle_scalar: 0.0,
                    }
                ), true).await?;
                factory.enable_compression();
                return Ok(ServerPacketHandlerStatus::Receiving);
            }
            MinecraftPacket::Login(login) => {
                if factory.status == PacketFactoryStatus::Logging {
                    println!("login packet");
                    let auth = login.auth;
                    let verified = login.verified;
                    /*
                    if server_data.properties.xbox_auth {//如果需要进行登录验证
                        if !auth || !verified {
                            println!("xbox 验证失败 auth: {}, verified: {}", auth, verified);
                            self.player_connection.disconnect("disconnectionScreen.notAuthenticated").await.unwrap();
                            return;
                        }
                        println!("xbox 验证成功");
                    }else {
                        println!("xbox 验证已关闭")
                    }

                     */
                    let temp = login.username.as_str();
                    if temp == "rcon" || temp == "console" {
                        factory.disconnect("disconnectionScreen.invalidName", false).await.unwrap();
                        //self.disconnected = true;
                        return Ok(ServerPacketHandlerStatus::Closed);
                    }
                    factory.data.username = login.username;
                    factory.data.uuid = login.identity;
                    factory.data.client_id = login.client_id;
                    factory.data.skin = Some(login.skin);
                    factory.data.device_os = login.device_os;
                    factory.data.device_model = login.device_model;
                    factory.data.language_code = login.language_code;
                    factory.data.game_version = login.game_version;
                    factory.data.server_address = login.server_address;
                    let identity_public_key = login.identity_public_key;
                    let identity_public_key = identity_public_key.as_str();
                    factory.status = PacketFactoryStatus::Handshaking;

                    //生成jwt
                    let key_pair = MinecraftEncryption::get_key_pair();
                    let token = MinecraftEncryption::create_token();
                    let client_key = MinecraftEncryption::parse_key(identity_public_key).unwrap();
                    let secret_key = MinecraftEncryption::get_secret_key(key_pair.clone().0, client_key, token).unwrap();
                    let jwt = MinecraftEncryption::create_handshake_jwt(key_pair, token).unwrap();
                    println!("jwt: {}", jwt);

                    //发送包
                    factory.send_packet(MinecraftPacket::ServerToClientHandshake(
                        ServerToClientHandshake {
                            jwt,
                        }
                    ), true).await?;
                    //启用加密
                    factory.enable_encryption(secret_key)?;
                    return Ok(ServerPacketHandlerStatus::Receiving);
                }
            }
            MinecraftPacket::ClientToServerHandshake(_) => {
                if factory.status == PacketFactoryStatus::Handshaking {
                    println!("client to server handshake");

                    //登录状态
                    let mut status = PlayStatus::LOGIN_SUCCESS;
                    let min_version = ProtocolInfo::get_global().unwrap().protocol_versions.get_min_version().unwrap();
                    let max_version = ProtocolInfo::get_global().unwrap().protocol_versions.get_min_version().unwrap();
                    if factory.protocol_version < min_version {
                        status = PlayStatus::LOGIN_FAILED_CLIENT;//过时的客户端
                        factory.disconnect = true;
                    } else if factory.protocol_version > max_version {
                        status = PlayStatus::LOGIN_FAILED_SERVER;//过时的服务端
                        factory.disconnect = true;
                    }

                    //发送包
                    factory.send_packet(MinecraftPacket::PlayStatus(
                        PlayStatus {
                            status
                        }
                    ), true).await?;

                    if factory.disconnect {
                        return Ok(ServerPacketHandlerStatus::Closed);
                    }
                    //加载材质包阶段
                    factory.status = PacketFactoryStatus::ResourcePack;

                    //发送包

                    //let resource_packs = server_data.resource_packs_manager.get_resource_packs().clone();
                    factory.send_packet(MinecraftPacket::ResourcePackInfo(
                        ResourcePackInfo {
                            must_accept: true,
                            scripting: false,
                            has_addon_packs: false,
                            force_server_packs: false,
                            behavior_packs: vec![],
                            resource_packs: vec![],
                        }
                    ), true).await?;
                }
                return Ok(ServerPacketHandlerStatus::Receiving);
            }
            MinecraftPacket::ResourcePackClientResponse(packet) => {
                println!("resource pack client response");
                println!("response status: {}", packet.response_status);
                match packet.response_status {
                    ResourcePackClientResponse::STATUS_HAVE_ALL_PACKS => {
                        println!("have all packs");
                        //设置实验玩法
                        /*
                        let server_properties = ServerData::get_global().unwrap();
                        let server_properties = server_properties.properties.clone();

                         */
                        let mut experiments = vec![];
                        experiments.push(
                            ExperimentData::DATA_DRIVEN_ITEMS.set_enabled(true)
                        );
                        experiments.push(
                            ExperimentData::DATA_DRIVEN_BIOMES.set_enabled(true)
                        );
                        experiments.push(
                            ExperimentData::UPCOMING_CREATOR_FEATURES.set_enabled(true)
                        );
                        experiments.push(
                            ExperimentData::GAMETEST.set_enabled(true)
                        );
                        experiments.push(
                            ExperimentData::EXPERIMENTAL_MOLOANG_FEATURE.set_enabled(true)
                        );
                        experiments.push(
                            ExperimentData::CAMERAS.set_enabled(true)
                        );
                        //发送包
                        factory.send_packet(MinecraftPacket::ResourcePackStack(ResourcePackStack {
                            must_accept: false,
                            behavior_pack_stack: vec![],
                            resource_pack_stack: vec![],
                            experiments,
                            game_version: "*".to_string(),
                            is_has_editor_packs: false,
                        }), true).await?;
                    },
                    ResourcePackClientResponse::STATUS_COMPLETED => {
                        println!("completed");
                        if factory.status == PacketFactoryStatus::ResourcePack {
                            factory.status = PacketFactoryStatus::PreSpawn;
                            return Ok(ServerPacketHandlerStatus::CreatePlayer);
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(ServerPacketHandlerStatus::Receiving);
    }
}