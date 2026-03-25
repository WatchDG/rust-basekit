use super::decode_full_group_into::decode_full_group_into;
use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

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
    let mut src_offset = 0usize;

    #[cfg(feature = "simd-ssse3")]
    {
        let simd_bytes = (full_groups / 4) * 16;
        if simd_bytes > 0 {
            dst_offset += unsafe {
                ssse3_decode_full_groups_into(
                    config,
                    &mut dst[..simd_bytes / 4 * 3],
                    &src[..simd_bytes],
                )
            }?;
            src_offset = simd_bytes;
        }
    }

    for chunk_start in (src_offset..full_groups * 4).step_by(4) {
        let chunk = &src[chunk_start..chunk_start + 4];
        dst_offset +=
            unsafe { decode_full_group_into(config, chunk, chunk_start, dst, dst_offset)? };
    }

    Ok(dst_offset)
}
