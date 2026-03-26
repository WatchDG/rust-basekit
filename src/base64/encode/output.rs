#[derive(PartialEq, Eq, Hash)]
pub struct Base64EncodeOutput {
    pub inner: Vec<u8>,
}

impl From<Base64EncodeOutput> for Vec<u8> {
    fn from(value: Base64EncodeOutput) -> Self {
        value.inner
    }
}

impl From<Base64EncodeOutput> for String {
    fn from(value: Base64EncodeOutput) -> Self {
        String::from_utf8(value.inner).expect("Base64 output is valid UTF-8")
    }
}
