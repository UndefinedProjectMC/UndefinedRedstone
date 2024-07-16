use undefined_redstone_protocol::encryption::MinecraftEncryption;
use undefined_redstone_protocol::{MinecraftPacket, ProtocolInfo};
use undefined_redstone_protocol::client::resource_packs::ResourcePackClientResponse;
use undefined_redstone_protocol::server::handshake::{NetworkSettings, ServerToClientHandshake};
use undefined_redstone_protocol::server::PlayStatus;
use undefined_redstone_protocol::server::resource_packs::{ExperimentData, ResourcePackInfo, ResourcePackStack};
use crate::packet::packet_factory::{PacketFactory, PacketFactoryStatus};
pub struct ServerPacketHandler;

impl ServerPacketHandler {

    pub async fn handle_packet(factory: &mut PacketFactory, packet: MinecraftPacket) -> anyhow::Result<()> {
        match packet {
            MinecraftPacket::RequestNetworkSettings(request) => {
                factory.status = PacketFactoryStatus::Logging;
                let protocol_version = request.protocol_version;
                println!("Protocol Version: {}", protocol_version);
                factory.protocol_version = protocol_version;
                let compression_algorithm = factory.settings.0.compression_algorithm;
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
                return Ok(());
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
                        return Ok(());
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
                    return Ok(());
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
                        return Ok(());
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
                return Ok(());
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
                            ExperimentData::new("data_driven_items", true)
                        );
                        experiments.push(
                            ExperimentData::new("data_driven_biomes", true)
                        );
                        experiments.push(
                            ExperimentData::new("upcoming_creator_features", true)
                        );
                        experiments.push(
                            ExperimentData::new("gametest", true)
                        );
                        experiments.push(
                            ExperimentData::new("experimental_molang_features", true)
                        );
                        experiments.push(
                            ExperimentData::new("cameras", true)
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
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        return Ok(());
    }
}