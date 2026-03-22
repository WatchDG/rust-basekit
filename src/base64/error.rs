#[derive(Debug, PartialEq)]
pub enum Base64Error {
    InvalidCharacter(u8, usize),
    InvalidPadding,
    InvalidLength(usize),
    DestinationBufferTooSmall { needed: usize, provided: usize },
}
