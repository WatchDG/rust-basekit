use core::ptr;

use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

#[cfg(feature = "simd-avx2")]
use super::simd::avx2::encode_full_groups_into_avx2;
#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::encode_full_groups_into_ssse3;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn encode_full_groups_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut dst_offset = 0usize;

    #[cfg(any(feature = "simd-avx2", feature = "simd-ssse3"))]
    let mut src_offset = 0usize;
    #[cfg(not(any(feature = "simd-avx2", feature = "simd-ssse3")))]
    let src_offset = 0usize;

    #[cfg(feature = "simd-avx2")]
    {
        let avx2_groups = if src.len() >= 48 { (src.len() - 24) / 24 } else { 0 };
        let avx2_bytes = avx2_groups * 24;

        if avx2_bytes > 0 {
            dst_offset += encode_full_groups_into_avx2(
                config,
                &mut dst[..avx2_groups * 32],
                &src[..avx2_bytes],
            )?;
            src_offset = avx2_bytes;
        }
    }

    #[cfg(feature = "simd-ssse3")]
    {
        let remaining = &src[src_offset..];
        let ssse3_groups = if remaining.len() >= 24 { remaining.len() / 12 - 1 } else { 0 };
        let ssse3_bytes = ssse3_groups * 12;

        if ssse3_bytes > 0 {
            dst_offset += encode_full_groups_into_ssse3(
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

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn encode_tail_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let padding = config.padding;

    match src.len() {
        1 => {
            let triple = (src[0] as u32) << 16;
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;
            let ptr = dst.as_mut_ptr();

            ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
            ptr.offset(1)
                .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
            ptr.offset(2).write(padding);
            ptr.offset(3).write(padding);
            Ok(4)
        }
        2 => {
            let triple = ((src[0] as u32) << 16) | ((src[1] as u32) << 8);
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;
            let c2 = ((triple >> 6) & 0x3F) as usize;
            let ptr = dst.as_mut_ptr();

            ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
            ptr.offset(1)
                .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
            ptr.offset(2)
                .write(ptr::read_unaligned(alphabet_ptr.add(c2)));
            ptr.offset(3).write(padding);
            Ok(4)
        }
        0 => Ok(0),
        _ => unreachable!(),
    }
}

#[inline(always)]
pub fn encode_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups = src.len() / 3;
    let tail_len = src.len() % 3;
    let output_len = full_groups * 4
        + match tail_len {
            0 => 0,
            1 | 2 => 4,
            _ => unreachable!(),
        };

    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    unsafe {
        let mut dst_offset = 0usize;

        dst_offset +=
            encode_full_groups_into(config, &mut dst[..full_groups * 4], &src[..full_groups * 3])?;

        if tail_len > 0 {
            dst_offset +=
                encode_tail_into(config, &mut dst[dst_offset..][..4], &src[full_groups * 3..])?;
        }

        Ok(dst_offset)
    }
}
