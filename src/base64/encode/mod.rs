pub mod encode_impl;
pub mod encode_into;

#[cfg(feature = "simd-sse3")]
pub mod simd;

pub use encode_impl::encode;
pub use encode_into::encode_into;
