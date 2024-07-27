use binary_util::BinaryIo;

#[derive(Clone, Debug, BinaryIo)]
pub struct RequestNetworkSettings {
    pub protocol_version: u32,
}
#[derive(Clone, Debug, BinaryIo)]
pub struct ClientToServerHandshake {}

