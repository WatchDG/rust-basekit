mod decode_full_groups_into;
mod decode_impl;
mod decode_into;
mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
mod simd;

pub use decode_impl::decode16;
pub use decode_into::decode16_into;
pub use output::Base16DecodeOutput;
