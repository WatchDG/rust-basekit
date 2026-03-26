pub struct Base32EncodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base32EncodeOutput> for Vec<u8> {
    fn from(value: Base32EncodeOutput) -> Self {
        value.inner
    }
}

impl From<Base32EncodeOutput> for String {
    fn from(value: Base32EncodeOutput) -> Self {
        String::from_utf8(value.inner).expect("Base32 output is valid UTF-8")
    }
}
