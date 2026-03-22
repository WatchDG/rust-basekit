#[derive(Debug)]
pub enum Base64Error {
    InvalidCharacter(u8, usize),
    InvalidPadding,
    InvalidLength(usize),
}
