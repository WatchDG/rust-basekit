pub struct Base64EncodeConfig {
    pub alphabet: &'static [u8; 64],
    pub padding: Option<u8>,
}

impl Base64EncodeConfig {
    pub fn new(alphabet: &'static [u8; 64], padding: Option<u8>) -> Self {
        Self { alphabet, padding }
    }
}

pub struct Base64DecodeConfig {
    pub decode_table: &'static [i8; 128],
    pub padding: u8,
}

impl Base64DecodeConfig {
    pub fn new(decode_table: &'static [i8; 128], padding: u8) -> Self {
        Self {
            decode_table,
            padding,
        }
    }
}
