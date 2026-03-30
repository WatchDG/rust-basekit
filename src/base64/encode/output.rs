#[derive(PartialEq, Eq, Hash)]
pub struct Base64EncodeOutput {
    pub(crate) inner: Vec<u8>,
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

impl AsRef<[u8]> for Base64EncodeOutput {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
