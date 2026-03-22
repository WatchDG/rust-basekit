pub struct Base64Config {
    pub alphabet: &'static [u8; 64],
    pub padding: u8,
}

impl Base64Config {
    pub fn new(alphabet: &'static [u8; 64], padding: u8) -> Self {
        Self { alphabet, padding }
    }
}
