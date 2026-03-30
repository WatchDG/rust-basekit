pub mod decode_full_group_into;
pub mod decode_full_groups_into;
pub mod decode_impl;
pub mod decode_into;
pub mod decode_tail_into;
pub mod output;
#[cfg(any(feature = "simd-ssse3", feature = "simd-avx2", feature = "simd-avx512"))]
pub mod simd;

pub(crate) use decode_full_groups_into::decode_full_groups_into;
pub use decode_impl::decode;
pub use decode_into::decode_into;
pub(crate) use decode_tail_into::decode_tail_into;
pub use output::Base32DecodeOutput;
