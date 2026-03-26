#[derive(PartialEq, Eq, Hash)]
pub struct Base16DecodeOutput {
    pub(crate) inner: Vec<u8>,
}

impl From<Base16DecodeOutput> for Vec<u8> {
    fn from(value: Base16DecodeOutput) -> Self {
        value.inner
    }
}

impl TryFrom<Base16DecodeOutput> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from(value: Base16DecodeOutput) -> Result<Self, Self::Error> {
        String::from_utf8(value.inner)
    }
}
