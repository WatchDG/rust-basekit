mod decode_impl;
mod decode_into;
mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
mod simd;

pub use decode_impl::decode;
pub use decode_into::decode_into;
pub use output::Base16DecodeOutput;
