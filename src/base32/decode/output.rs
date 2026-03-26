pub struct Base32DecodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base32DecodeOutput> for Vec<u8> {
    fn from(value: Base32DecodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base32DecodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base32DecodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}
