use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx2"
))]
use crate::cpu_features::is_available_feature_simd_avx2;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx512"
))]
use crate::cpu_features::is_available_feature_simd_avx512;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use crate::cpu_features::is_available_feature_simd_ssse3;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx2"
))]
use super::simd::avx2::avx2_encode_full_groups_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx512"
))]
use super::simd::avx512::avx512_encode_full_groups_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use super::simd::ssse3::ssse3_encode_full_groups_into;

use super::encode_full_group_into;

#[inline(always)]
pub(crate) fn encode_full_groups_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0, "src length must be a multiple of 5");

    if src.is_empty() {
        return Ok(0);
    }

    let full_groups_count = src.len() / 5;
    let output_len = full_groups_count * 8;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut dst_offset = 0usize;

    #[cfg(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-avx512"
        ),
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-avx2"
        ),
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-ssse3"
        )
    ))]
    let mut src_offset = 0usize;
    #[cfg(not(any(
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-avx512"
        ),
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-avx2"
        ),
        all(
            any(target_arch = "x86", target_arch = "x86_64"),
            feature = "simd-ssse3"
        )
    )))]
    let src_offset = 0usize;

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx512"
    ))]
    if is_available_feature_simd_avx512() {
        let written = unsafe {
            avx512_encode_full_groups_into(config, &mut dst[dst_offset..], &src[src_offset..])?
        };
        src_offset += written * 5 / 8;
        dst_offset += written;
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx2"
    ))]
    if is_available_feature_simd_avx2() {
        let written = unsafe {
            avx2_encode_full_groups_into(config, &mut dst[dst_offset..], &src[src_offset..])?
        };
        src_offset += written * 5 / 8;
        dst_offset += written;
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-ssse3"
    ))]
    if is_available_feature_simd_ssse3() {
        let written = unsafe {
            ssse3_encode_full_groups_into(config, &mut dst[dst_offset..], &src[src_offset..])?
        };
        src_offset += written * 5 / 8;
        dst_offset += written;
    }

    for full_group in src[src_offset..].chunks_exact(5) {
        dst_offset += encode_full_group_into(config, &mut dst[dst_offset..], full_group)?;
    }

    Ok(dst_offset)
}
