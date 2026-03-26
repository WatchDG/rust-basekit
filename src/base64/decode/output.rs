#[derive(PartialEq, Eq, Hash)]
pub struct Base64DecodeOutput {
    pub(crate) inner: Vec<u8>,
}

impl From<Base64DecodeOutput> for Vec<u8> {
    fn from(value: Base64DecodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base64DecodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base64DecodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}

impl AsRef<[u8]> for Base64DecodeOutput {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}
