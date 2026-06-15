mod encode_full_group_into;
mod encode_full_groups_into;
mod encode_impl;
mod encode_into;
mod encode_tail_into;
mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
mod simd;

pub(crate) use encode_full_group_into::encode_full_group_into;
pub use encode_impl::encode64;
pub use encode_into::encode64_into;
pub use output::Base64EncodeOutput;
