#[derive(Clone, Debug)]
pub struct Skin {
    skin_data: Vec<u8>,
    skin_id: String,
}

impl Skin {
    pub const SINGLE_SKIN_SIZE: usize = 64 * 32 * 4;
    pub const DOUBLE_SKIN_SIZE: usize = 64 * 64 * 4;
    
    pub fn new(skin_data: Vec<u8>, width: u32, height: u32, skin_id: &str) -> Option<Self> {
        Some(Self {
            skin_data,
            skin_id: skin_id.to_string()
        })
    }
    pub fn new_base64(base64: &str, width: u32, height: u32, skin_id: &str) -> Option<Self> {
        let base64_decode = openssl::base64::decode_block(base64).ok()?;
        Self::new(base64_decode, width, height, skin_id)
    }
}