use binary_util::BinaryIo;

pub mod login;
pub mod handshake;
pub mod resource_packs;

#[derive(Clone, Debug, BinaryIo)]
pub struct ClientCacheStatus {
    pub supported: bool
}