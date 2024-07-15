use std::sync::Arc;
use binary_util::BinaryIo;
use undefined_redstone_type::minecraft_version::MinecraftVersions;
use undefined_redstone_type::protocol_versions::MinecraftProtocolVersions;
use crate::client::*;
use crate::client::login::*;
use crate::client::handshake::*;
use crate::client::resource_packs::*;
use crate::server::*;
use crate::server::handshake::*;
use crate::server::resource_packs::*;

pub mod client;
pub mod server;
pub mod encryption;
pub mod skin;

static mut GLOBAL_PROTOCOL_INFO: Option<Arc<ProtocolInfo>> = None;

pub struct ProtocolInfo {
    pub protocol_versions: MinecraftProtocolVersions,
    pub minecraft_versions: MinecraftVersions,
}

impl ProtocolInfo {
    pub fn new_global(protocol_versions: MinecraftProtocolVersions, minecraft_versions: MinecraftVersions) {
        unsafe {
            GLOBAL_PROTOCOL_INFO = Some(Arc::new(ProtocolInfo {
                protocol_versions,
                minecraft_versions,
            }));
        }
    }

    pub fn get_global() -> Option<Arc<ProtocolInfo>> {
        unsafe {
            GLOBAL_PROTOCOL_INFO.clone()
        }
    }
}

#[derive(Clone, Debug, BinaryIo)]
#[repr(u8)]
pub enum MinecraftPacket {
    Login(Login) = 0x01,
    PlayStatus(PlayStatus) = 0x02,
    ServerToClientHandshake(ServerToClientHandshake) = 0x03,
    ClientToServerHandshake(ClientToServerHandshake) = 0x04,
    Disconnect(Disconnect) = 0x05,
    ResourcePackInfo(ResourcePackInfo) = 0x06,
    ResourcePackStack(ResourcePackStack) = 0x07,
    ResourcePackClientResponse(ResourcePackClientResponse) = 0x08,
    RequestNetworkSettings(RequestNetworkSettings) = 0xc1,
    ClientCacheStatus(ClientCacheStatus) = 0x81,
    NetworkSettings(NetworkSettings) = 0x8f,
}