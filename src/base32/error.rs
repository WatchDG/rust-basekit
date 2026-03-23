#[derive(Debug, PartialEq)]
pub enum Base32Error {
    InvalidCharacter(u8, usize),
    InvalidPadding,
    InvalidLength(usize),
    DestinationBufferTooSmall { needed: usize, provided: usize },
}
