pub mod decode_impl;
pub mod decode_into;
pub mod output;
pub mod simd;

pub use decode_impl::decode;
pub use decode_into::decode_into;
pub use output::Base16DecodeOutput;
