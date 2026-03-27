use super::super::config::Base16EncodeConfig;
use super::super::error::Base16Error;
use super::encode_full_group_into::encode_full_group_into;

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
use super::simd::avx2::avx2_encode_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx512"
))]
use super::simd::avx512::avx512_encode_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use super::simd::ssse3::ssse3_encode_into;

#[inline(always)]
pub fn encode_into(
    config: &Base16EncodeConfig,
    mut dst: impl AsMut<[u8]>,
    src: impl AsRef<[u8]>,
) -> Result<usize, Base16Error> {
    let src = src.as_ref();
    if src.is_empty() {
        return Ok(0);
    }

    let output_len = src.len() * 2;

    let dst = dst.as_mut();
    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
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
        let avx512_groups = src.len() / 32;
        let avx512_bytes = avx512_groups * 32;

        if avx512_bytes > 0 {
            dst_offset += unsafe {
                avx512_encode_into(config, &mut dst[..avx512_groups * 64], &src[..avx512_bytes])?
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
        let avx2_groups = remaining.len() / 16;
        let avx2_bytes = avx2_groups * 16;

        if avx2_bytes > 0 {
            dst_offset += unsafe {
                avx2_encode_into(
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
        let ssse3_groups = remaining.len() / 8;
        let ssse3_bytes = ssse3_groups * 8;

        if ssse3_bytes > 0 {
            dst_offset += unsafe {
                ssse3_encode_into(
                    config,
                    &mut dst[dst_offset..dst_offset + ssse3_groups * 16],
                    &remaining[..ssse3_bytes],
                )?
            };
            src_offset += ssse3_bytes;
        }
    }

    for chunk in src[src_offset..].chunks_exact(1) {
        dst_offset += encode_full_group_into(config, &mut dst[dst_offset..], chunk)?;
    }

    Ok(output_len)
}
