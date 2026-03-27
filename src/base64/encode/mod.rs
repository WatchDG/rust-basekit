pub mod encode_full_group_into;
pub mod encode_full_groups_into;
pub mod encode_impl;
pub mod encode_into;
pub mod encode_tail_into;
pub mod output;

#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
pub mod simd;

pub(crate) use encode_full_group_into::encode_full_group_into;
pub(crate) use encode_full_groups_into::encode_full_groups_into;
pub use encode_impl::encode;
pub use encode_into::encode_into;
pub use encode_tail_into::encode_tail_into;
pub use output::Base64EncodeOutput;
