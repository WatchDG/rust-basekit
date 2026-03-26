pub struct Base16EncodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base16EncodeOutput> for Vec<u8> {
    fn from(value: Base16EncodeOutput) -> Self {
        value.inner
    }
}

impl From<Base16EncodeOutput> for String {
    fn from(value: Base16EncodeOutput) -> Self {
        String::from_utf8(value.inner).expect("Base16 output is valid UTF-8")
    }
}
