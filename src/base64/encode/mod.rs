pub mod encode_impl;
pub mod encode_into;

#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2"))]
pub mod simd;

pub use encode_impl::encode;
pub use encode_into::encode_into;
