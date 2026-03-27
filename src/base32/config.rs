pub struct Base32EncodeConfig {
    pub alphabet: &'static [u8; 32],
    pub padding: Option<u8>,
}

impl Base32EncodeConfig {
    pub fn new(alphabet: &'static [u8; 32], padding: Option<u8>) -> Self {
        Self { alphabet, padding }
    }
}

pub struct Base32DecodeConfig {
    pub decode_table: &'static [i8; 128],
    pub padding: Option<u8>,
}

impl Base32DecodeConfig {
    pub fn new(decode_table: &'static [i8; 128], padding: Option<u8>) -> Self {
        Self {
            decode_table,
            padding,
        }
    }
}
