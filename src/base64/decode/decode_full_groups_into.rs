use super::decode_full_group_into::decode_full_group_into;
use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

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
use super::simd::avx2::avx2_decode_full_groups_into;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx512"
))]
use super::simd::avx512::avx512_decode_full_groups_into;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use super::simd::ssse3::ssse3_decode_full_groups_into;

#[inline(always)]
pub(crate) unsafe fn decode_full_groups_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.len() < 4 {
        return Ok(0);
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
        let avx512_src_bytes = (src.len() / 64) * 64;
        if avx512_src_bytes > 0 {
            let avx512_dst_bytes = avx512_src_bytes / 4 * 3;
            dst_offset += unsafe {
                avx512_decode_full_groups_into(
                    config,
                    &mut dst[dst_offset..dst_offset + avx512_dst_bytes],
                    &src[src_offset..src_offset + avx512_src_bytes],
                )
            }?;
            src_offset += avx512_src_bytes;
        }
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx2"
    ))]
    if is_available_feature_simd_avx2() {
        let avx2_src_bytes = ((src.len() - src_offset) / 32) * 32;
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

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-ssse3"
    ))]
    if is_available_feature_simd_ssse3() {
        let ssse3_src_bytes = ((src.len() - src_offset) / 16) * 16;
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

    for src_offset in (src_offset..src.len()).step_by(4) {
        let full_group_src = &src[src_offset..src_offset + 4];
        dst_offset +=
            decode_full_group_into(config, &mut dst[dst_offset..], full_group_src, src_offset)?;
    }

    Ok(dst_offset)
}
