use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;
use super::decode_full_group_into::decode_full_group_into;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::avx2_decode_full_groups_into;

#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_decode_full_groups_into;

#[inline(always)]
pub fn decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    full_groups: usize,
) -> Result<usize, Base32Error> {
    if full_groups == 0 {
        return Ok(0);
    }

    let mut dst_offset = 0usize;

    #[cfg(any(feature = "simd-avx2", feature = "simd-ssse3"))]
    let mut src_offset = 0usize;
    #[cfg(not(any(feature = "simd-avx2", feature = "simd-ssse3")))]
    let src_offset = 0usize;

    #[cfg(feature = "simd-avx2")]
    {
        // Process 4 full groups (32 src bytes → 20 dst bytes) per AVX2 iteration.
        let avx2_groups = full_groups / 4;
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

    #[cfg(feature = "simd-ssse3")]
    {
        // Process 2 full groups (16 src bytes → 10 dst bytes) per SSSE3 iteration.
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

    for chunk_start in (src_offset..full_groups * 8).step_by(8) {
        let chunk = &src[chunk_start..chunk_start + 8];
        dst_offset +=
            unsafe { decode_full_group_into(config, chunk, chunk_start, dst, dst_offset)? };
    }

    Ok(dst_offset)
}
