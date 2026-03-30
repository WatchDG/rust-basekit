use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;

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
use super::simd::avx2::avx2_decode_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-avx512"
))]
use super::simd::avx512::avx512_decode_into;
#[cfg(all(
    any(target_arch = "x86", target_arch = "x86_64"),
    feature = "simd-ssse3"
))]
use super::simd::ssse3::ssse3_decode_into;

#[inline]
pub fn decode_into(
    config: &Base16DecodeConfig,
    mut dst: impl AsMut<[u8]>,
    src: impl AsRef<[u8]>,
) -> Result<usize, Base16Error> {
    let src = src.as_ref();
    if src.is_empty() {
        return Ok(0);
    }

    if !src.len().is_multiple_of(2) {
        return Err(Base16Error::InvalidLength(src.len()));
    }

    let output_len = src.len() / 2;

    let dst = dst.as_mut();
    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx512"
    ))]
    if is_available_feature_simd_avx512() {
        let written =
            unsafe { avx512_decode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written * 2;
        dst_offset += written;
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-avx2"
    ))]
    if is_available_feature_simd_avx2() {
        let written =
            unsafe { avx2_decode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written * 2;
        dst_offset += written;
    }

    #[cfg(all(
        any(target_arch = "x86", target_arch = "x86_64"),
        feature = "simd-ssse3"
    ))]
    if is_available_feature_simd_ssse3() {
        let written =
            unsafe { ssse3_decode_into(config, &mut dst[dst_offset..], &src[src_offset..]) };
        src_offset += written * 2;
        dst_offset += written;
    }

    let decode_table = config.decode_table;

    unsafe {
        while src_offset < src.len() {
            let high_nibble = src[src_offset];
            let low_nibble = src[src_offset + 1];

            if high_nibble >= 128 {
                return Err(Base16Error::InvalidCharacter(high_nibble, src_offset));
            }
            if low_nibble >= 128 {
                return Err(Base16Error::InvalidCharacter(low_nibble, src_offset + 1));
            }

            let high_val = decode_table[high_nibble as usize];
            let low_val = decode_table[low_nibble as usize];

            if high_val < 0 {
                return Err(Base16Error::InvalidCharacter(high_nibble, src_offset));
            }
            if low_val < 0 {
                return Err(Base16Error::InvalidCharacter(low_nibble, src_offset + 1));
            }

            let ptr = dst.as_mut_ptr().add(dst_offset);
            ptr.write(((high_val as u8) << 4) | (low_val as u8));

            src_offset += 2;
            dst_offset += 1;
        }

        Ok(output_len)
    }
}
