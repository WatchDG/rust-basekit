mod config;
mod consts;
mod decode;
mod encode;
mod error;

pub use config::{Base16DecodeConfig, Base16EncodeConfig};
pub use consts::{
    ALPHABET_BASE16_LOWERCASE, ALPHABET_BASE16_UPPERCASE, DECODE_TABLE_BASE16_LOWERCASE,
    DECODE_TABLE_BASE16_UPPERCASE,
};
pub use decode::{Base16DecodeOutput, decode16, decode16_into};
pub use encode::{Base16EncodeOutput, encode16, encode16_into};
pub use error::Base16Error;
