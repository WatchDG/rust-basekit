use super::super::config::Base16EncodeConfig;
use super::super::error::Base16Error;
use super::encode_full_group_into::encode_full_group_into;

#[cfg(feature = "simd-avx2")]
use crate::cpu_features::is_available_feature_simd_avx2;
#[cfg(feature = "simd-avx512")]
use crate::cpu_features::is_available_feature_simd_avx512;
#[cfg(feature = "simd-ssse3")]
use crate::cpu_features::is_available_feature_simd_ssse3;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::avx2_encode_into;
#[cfg(feature = "simd-avx512")]
use super::simd::avx512::avx512_encode_into;
#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_encode_into;

#[inline(always)]
pub fn encode_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let output_len = src.len() * 2;

    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    #[cfg(feature = "simd-avx512")]
    if is_available_feature_simd_avx512() {
        let written =
            unsafe { avx512_encode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written / 2;
        dst_offset += written;
    }

    #[cfg(feature = "simd-avx2")]
    if is_available_feature_simd_avx2() {
        let written =
            unsafe { avx2_encode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written / 2;
        dst_offset += written;
    }

    #[cfg(feature = "simd-ssse3")]
    if is_available_feature_simd_ssse3() {
        let written =
            unsafe { ssse3_encode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written / 2;
        dst_offset += written;
    }

    dst_offset +=
        unsafe { encode_full_group_into(config, &mut dst[dst_offset..], &src[src_offset..]) };

    Ok(output_len)
}
