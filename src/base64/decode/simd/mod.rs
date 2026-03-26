#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx2"
))]
pub mod avx2;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
pub mod ssse3;
