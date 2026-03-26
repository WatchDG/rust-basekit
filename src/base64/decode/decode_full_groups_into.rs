use super::decode_full_group_into::decode_full_group_into;
use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

#[cfg(feature = "simd-avx2")]
use crate::cpu_features::is_available_feature_simd_avx2;
#[cfg(feature = "simd-ssse3")]
use crate::cpu_features::is_available_feature_simd_ssse3;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::avx2_decode_full_groups_into;

#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_decode_full_groups_into;

#[inline(always)]
pub(crate) unsafe fn decode_full_groups_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    full_groups: usize,
) -> Result<usize, Base64Error> {
    if full_groups == 0 {
        return Ok(0);
    }

    let mut dst_offset = 0usize;

    #[cfg(any(feature = "simd-avx2", feature = "simd-ssse3"))]
    let mut src_offset = 0usize;
    #[cfg(not(any(feature = "simd-avx2", feature = "simd-ssse3")))]
    let src_offset = 0usize;

    #[cfg(feature = "simd-avx2")]
    if is_available_feature_simd_avx2() {
        let avx2_src_bytes = (full_groups / 8) * 32;
        if avx2_src_bytes > 0 {
            let avx2_dst_bytes = avx2_src_bytes / 4 * 3;
            dst_offset += unsafe {
                avx2_decode_full_groups_into(
                    config,
                    &mut dst[dst_offset..dst_offset + avx2_dst_bytes],
                    &src[src_offset..src_offset + avx2_src_bytes],
                )
            }?;
            src_offset += avx2_src_bytes;
        }
    }

    #[cfg(feature = "simd-ssse3")]
    if is_available_feature_simd_ssse3() {
        let remaining_groups = full_groups - src_offset / 4;
        let ssse3_src_bytes = (remaining_groups / 4) * 16;
        if ssse3_src_bytes > 0 {
            let ssse3_dst_bytes = ssse3_src_bytes / 4 * 3;
            dst_offset += unsafe {
                ssse3_decode_full_groups_into(
                    config,
                    &mut dst[dst_offset..dst_offset + ssse3_dst_bytes],
                    &src[src_offset..src_offset + ssse3_src_bytes],
                )
            }?;
            src_offset += ssse3_src_bytes;
        }
    }

    for chunk_start in (src_offset..full_groups * 4).step_by(4) {
        let chunk = &src[chunk_start..chunk_start + 4];
        dst_offset +=
            unsafe { decode_full_group_into(config, chunk, chunk_start, dst, dst_offset)? };
    }

    Ok(dst_offset)
}
