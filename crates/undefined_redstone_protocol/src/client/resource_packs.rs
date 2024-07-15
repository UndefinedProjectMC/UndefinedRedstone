use std::io::Error;
use binary_util::{ByteReader, ByteWriter};
use binary_util::interfaces::{Reader, Writer};

#[derive(Clone, Debug)]
struct ResponseEntry {
    uuid: String,
    version: String
}

#[derive(Clone, Debug)]
pub struct ResourcePackClientResponse {
    pub response_status: u8,
    pub entries: Vec<ResponseEntry>
}

impl ResourcePackClientResponse {
    pub const STATUS_HAVE_ALL_PACKS: u8 = 3;
    pub const STATUS_COMPLETED: u8 = 4;
    pub fn new() -> Self {
        Self {
            response_status: 0,
            entries: vec![],
        }
    }
}

impl Writer for ResourcePackClientResponse {
    fn write(&self, buf: &mut ByteWriter) -> Result<(), Error> {
        Ok(())
    }
}

impl Reader<ResourcePackClientResponse> for ResourcePackClientResponse {
    fn read(buf: &mut ByteReader) -> Result<ResourcePackClientResponse, Error> {
        let mut packet = ResourcePackClientResponse::new();
        packet.response_status = buf.read_u8()?;
        let entries_len = buf.read_i16_le()? as u16;
        for _ in 0..entries_len {
            let entry = buf.read_string()?;
            let entry: Vec<&str> = entry.split("_").collect();
            packet.entries.push(ResponseEntry {
                uuid: entry.get(0).ok_or(Error::other("Cannot get uuid"))?.to_string(),
                version: entry.get(1).ok_or(Error::other("Cannot get version"))?.to_string(),
            })
        }
        Ok(packet)
    }
}