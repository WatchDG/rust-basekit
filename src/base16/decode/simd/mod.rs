#[cfg(feature = "simd-avx2")]
pub mod avx2;
#[cfg(feature = "simd-avx512")]
pub mod avx512;
#[cfg(feature = "simd-ssse3")]
pub mod ssse3;
