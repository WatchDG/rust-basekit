pub mod config;
pub mod consts;
pub mod decode;
pub mod encode;
pub mod error;

pub use config::{Base64DecodeConfig, Base64EncodeConfig};
pub use consts::{
    ALPHABET_BASE64, ALPHABET_BASE64_URL, DECODE_TABLE_BASE64, DECODE_TABLE_BASE64_URL,
    PADDING_BASE64,
};
pub use decode::{decode, decode_into};
pub use encode::{Base64EncodeOutput, encode, encode_into};
pub use error::Base64Error;
