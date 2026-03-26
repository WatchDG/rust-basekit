#[derive(PartialEq, Eq, Hash)]
pub struct Base32EncodeOutput {
    pub(crate) inner: Vec<u8>,
}

impl From<Base32EncodeOutput> for Vec<u8> {
    fn from(value: Base32EncodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base32EncodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base32EncodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}

impl AsRef<[u8]> for Base32EncodeOutput {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
