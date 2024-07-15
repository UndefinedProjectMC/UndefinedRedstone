pub mod handshake;
pub mod resource_packs;

use binary_util::BinaryIo;

#[derive(Clone, Debug, BinaryIo)]
pub struct PlayStatus {
    pub(crate) status: u32,
}

impl PlayStatus {
    pub const LOGIN_SUCCESS: u32 = 0;
    pub const LOGIN_FAILED_CLIENT: u32 = 1;
    pub const LOGIN_FAILED_SERVER: u32 = 2;
    pub const PLAYER_SPAWN: u32 = 3;
    pub const LOGIN_FAILED_INVALID_TENANT: u32 = 4;
    pub const LOGIN_FAILED_VANILLA_EDU: u32 = 5;
    pub const LOGIN_FAILED_EDU_VANILLA: u32 = 6;
    pub const LOGIN_FAILED_SERVER_FULL: u32 = 7;
    pub const LOGIN_FAILED_EDITOR_TO_VANILLA_MISMATCH: u32 = 8;
    pub const LOGIN_FAILED_VANILLA_TO_EDITOR_MISMATCH: u32 = 9;

}

#[derive(Clone, Debug, BinaryIo)]
pub struct Disconnect {
    pub hide_disconnect_screen: bool,
    pub kick_message: String,
}