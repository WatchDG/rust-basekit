pub struct Base16EncodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base16EncodeOutput> for Vec<u8> {
    fn from(value: Base16EncodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base16EncodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base16EncodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}
