pub struct Base16EncodeConfig {
    pub alphabet: &'static [u8; 16],
}

impl Base16EncodeConfig {
    pub fn new(alphabet: &'static [u8; 16]) -> Self {
        Self { alphabet }
    }
}

pub struct Base16DecodeConfig {
    pub decode_table: &'static [i8; 128],
}

impl Base16DecodeConfig {
    pub fn new(decode_table: &'static [i8; 128]) -> Self {
        Self { decode_table }
    }
}
