#[derive(Debug, PartialEq)]
pub enum Base16Error {
    InvalidCharacter(u8, usize),
    InvalidLength(usize),
    DestinationBufferTooSmall { needed: usize, provided: usize },
}
