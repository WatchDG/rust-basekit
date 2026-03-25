use core::ptr;

use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::avx2_encode_full_groups_into;
#[cfg(feature = "simd-avx512")]
use super::simd::avx512::avx512_encode_full_groups_into;
#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_encode_full_groups_into;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut dst_offset = 0usize;

    #[cfg(any(feature = "simd-avx512", feature = "simd-avx2", feature = "simd-ssse3"))]
    let mut src_offset = 0usize;
    #[cfg(not(any(feature = "simd-avx512", feature = "simd-avx2", feature = "simd-ssse3")))]
    let src_offset = 0usize;

    #[cfg(feature = "simd-avx512")]
    {
        let avx512_groups = if src.len() >= 96 {
            (src.len() - 48) / 48
        } else {
            0
        };
        let avx512_bytes = avx512_groups * 48;

        if avx512_bytes > 0 {
            dst_offset += avx512_encode_full_groups_into(
                config,
                &mut dst[..avx512_groups * 64],
                &src[..avx512_bytes],
            )?;
            src_offset = avx512_bytes;
        }
    }

    #[cfg(feature = "simd-avx2")]
    {
        let remaining = &src[src_offset..];
        let avx2_groups = if remaining.len() >= 48 {
            (remaining.len() - 24) / 24
        } else {
            0
        };
        let avx2_bytes = avx2_groups * 24;

        if avx2_bytes > 0 {
            dst_offset += avx2_encode_full_groups_into(
                config,
                &mut dst[dst_offset..dst_offset + avx2_groups * 32],
                &remaining[..avx2_bytes],
            )?;
            src_offset += avx2_bytes;
        }
    }

    #[cfg(feature = "simd-ssse3")]
    {
        let remaining = &src[src_offset..];
        let ssse3_groups = if remaining.len() >= 24 {
            remaining.len() / 12 - 1
        } else {
            0
        };
        let ssse3_bytes = ssse3_groups * 12;

        if ssse3_bytes > 0 {
            dst_offset += ssse3_encode_full_groups_into(
                config,
                &mut dst[dst_offset..dst_offset + ssse3_groups * 16],
                &remaining[..ssse3_bytes],
            )?;
            src_offset += ssse3_bytes;
        }
    }

    for chunk in src[src_offset..].chunks_exact(3) {
        let triple = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
        let ptr = dst.as_mut_ptr().add(dst_offset);

        ptr.write(ptr::read_unaligned(
            alphabet_ptr.add((triple >> 18 & 0x3F) as usize),
        ));
        ptr.offset(1).write(ptr::read_unaligned(
            alphabet_ptr.add((triple >> 12 & 0x3F) as usize),
        ));
        ptr.offset(2).write(ptr::read_unaligned(
            alphabet_ptr.add((triple >> 6 & 0x3F) as usize),
        ));
        ptr.offset(3).write(ptr::read_unaligned(
            alphabet_ptr.add((triple & 0x3F) as usize),
        ));

        dst_offset += 4;
    }

    Ok(dst_offset)
}
