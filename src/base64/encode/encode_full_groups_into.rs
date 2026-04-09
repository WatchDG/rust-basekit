use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;
use super::encode_full_group_into;

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

#[inline(always)]
pub(crate) fn encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
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
        let avx512_groups = src.len() / 48;
        let avx512_bytes = avx512_groups * 48;

        if avx512_groups >= 1 {
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

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx2"
    ))]
    if is_available_feature_simd_avx2() {
        let remaining = &src[src_offset..];
        let avx2_groups = remaining.len() / 24;
        let avx2_bytes = avx2_groups * 24;

        if avx2_groups >= 1 {
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

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-ssse3"
    ))]
    if is_available_feature_simd_ssse3() {
        let remaining = &src[src_offset..];
        let ssse3_groups = remaining.len() / 12;
        let ssse3_bytes = ssse3_groups * 12;

        if ssse3_groups >= 1 {
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

    for full_group in src[src_offset..].chunks_exact(3) {
        dst_offset += encode_full_group_into(config, &mut dst[dst_offset..], full_group)?;
    }

    Ok(dst_offset)
}
