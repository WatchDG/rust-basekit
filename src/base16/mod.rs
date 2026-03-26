pub mod config;
pub mod consts;
pub mod decode;
pub mod encode;
pub mod error;

pub use config::{Base16DecodeConfig, Base16EncodeConfig};
pub use consts::{
    ALPHABET_BASE16_LOWERCASE, ALPHABET_BASE16_UPPERCASE, DECODE_TABLE_BASE16_LOWERCASE,
    DECODE_TABLE_BASE16_UPPERCASE,
};
pub use decode::{decode, decode_into};
pub use encode::{Base16EncodeOutput, encode, encode_into};
pub use error::Base16Error;
