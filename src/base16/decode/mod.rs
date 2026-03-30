pub mod decode_impl;
pub mod decode_into;
pub mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
pub mod simd;

pub use decode_impl::decode;
pub use decode_into::decode_into;
pub use output::Base16DecodeOutput;
