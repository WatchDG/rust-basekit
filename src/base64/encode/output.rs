pub struct Base64EncodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base64EncodeOutput> for Vec<u8> {
    fn from(value: Base64EncodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base64EncodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base64EncodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}
