mod encode_full_group_into;
mod encode_full_groups_into;
mod encode_impl;
mod encode_into;
mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
mod simd;

pub use encode_impl::encode16;
pub use encode_into::encode16_into;
pub use output::Base16EncodeOutput;
