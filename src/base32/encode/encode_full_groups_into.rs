use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

#[cfg(feature = "simd-avx2")]
use crate::cpu_features::is_available_feature_simd_avx2;
#[cfg(feature = "simd-avx512")]
use crate::cpu_features::is_available_feature_simd_avx512;
#[cfg(feature = "simd-ssse3")]
use crate::cpu_features::is_available_feature_simd_ssse3;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::avx2_encode_full_groups_into;
#[cfg(feature = "simd-avx512")]
use super::simd::avx512::avx512_encode_full_groups_into;
#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_encode_full_groups_into;

use super::encode_full_group_into;

#[inline(always)]
pub fn encode_full_groups_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0, "src length must be a multiple of 5");

    if src.is_empty() {
        return Ok(0);
    }

    let full_chunks = src.len() / 5;
    let output_len = full_chunks * 8;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut dst_offset = 0usize;

    #[cfg(any(feature = "simd-avx512", feature = "simd-avx2", feature = "simd-ssse3"))]
    let mut src_offset = 0usize;
    #[cfg(not(any(feature = "simd-avx512", feature = "simd-avx2", feature = "simd-ssse3")))]
    let src_offset = 0usize;

    #[cfg(feature = "simd-avx512")]
    if is_available_feature_simd_avx512() {
        let avx512_groups = src.len() / 40;
        let avx512_bytes = avx512_groups * 40;

        if avx512_bytes > 0 {
            dst_offset += unsafe {
                avx512_encode_full_groups_into(
                    config,
                    &mut dst[..avx512_groups * 64],
                    &src[..avx512_bytes],
                )?
            };
            src_offset = avx512_bytes;
        }
    }

    #[cfg(feature = "simd-avx2")]
    if is_available_feature_simd_avx2() {
        let remaining = &src[src_offset..];
        let avx2_groups = remaining.len() / 20;
        let avx2_bytes = avx2_groups * 20;

        if avx2_bytes > 0 {
            dst_offset += unsafe {
                avx2_encode_full_groups_into(
                    config,
                    &mut dst[dst_offset..dst_offset + avx2_groups * 32],
                    &remaining[..avx2_bytes],
                )?
            };
            src_offset += avx2_bytes;
        }
    }

    #[cfg(feature = "simd-ssse3")]
    if is_available_feature_simd_ssse3() {
        let remaining = &src[src_offset..];
        let ssse3_groups = remaining.len() / 10;
        let ssse3_bytes = ssse3_groups * 10;

        if ssse3_bytes > 0 {
            dst_offset += unsafe {
                ssse3_encode_full_groups_into(
                    config,
                    &mut dst[dst_offset..dst_offset + ssse3_groups * 16],
                    &remaining[..ssse3_bytes],
                )?
            };
            src_offset += ssse3_bytes;
        }
    }

    for chunk in src[src_offset..].chunks_exact(5) {
        dst_offset += encode_full_group_into(config, &mut dst[dst_offset..], chunk);
    }

    Ok(dst_offset)
}
