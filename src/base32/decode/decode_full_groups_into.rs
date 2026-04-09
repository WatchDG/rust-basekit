use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;
use super::decode_full_group_into::decode_full_group_into;

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
    feature = "simd-avx512"
))]
use super::simd::avx512::avx512_decode_full_groups_into;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx2"
))]
use super::simd::avx2::avx2_decode_full_groups_into;

#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use super::simd::ssse3::ssse3_decode_full_groups_into;

#[inline(always)]
pub(crate) fn decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    full_groups: usize,
) -> Result<usize, Base32Error> {
    if full_groups == 0 {
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
        let avx512_groups = full_groups / 8;
        let avx512_src_bytes = avx512_groups * 64;
        if avx512_src_bytes > 0 {
            let avx512_dst_bytes = avx512_groups * 40;
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
        let remaining_groups = full_groups - src_offset / 8;
        let avx2_groups = remaining_groups / 4;
        let avx2_src_bytes = avx2_groups * 32;
        if avx2_src_bytes > 0 {
            let avx2_dst_bytes = avx2_groups * 20;
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
        let remaining_groups = full_groups - src_offset / 8;
        let ssse3_groups = remaining_groups / 2;
        let ssse3_src_bytes = ssse3_groups * 16;
        if ssse3_src_bytes > 0 {
            let ssse3_dst_bytes = ssse3_groups * 10;
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

    for src_offset in (src_offset..full_groups * 8).step_by(8) {
        let src_group = &src[src_offset..src_offset + 8];
        dst_offset +=
            unsafe { decode_full_group_into(config, dst, dst_offset, src_group, src_offset)? };
    }

    Ok(dst_offset)
}
