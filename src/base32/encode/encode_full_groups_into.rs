use core::ptr;

use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

#[cfg(feature = "simd-ssse3")]
use super::simd::ssse3::ssse3_encode_full_groups_into;

#[inline(always)]
pub fn encode_full_groups_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0, "src length must be a multiple of 5");

    if src.is_empty() {
        return Ok(0);
    }

    let full_chunks = src.len() / 5;
    let output_len = full_chunks * 8;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    #[cfg(feature = "simd-ssse3")]
    let mut src_offset = 0usize;
    #[cfg(not(feature = "simd-ssse3"))]
    let src_offset = 0usize;

    let mut dst_offset = 0usize;

    #[cfg(feature = "simd-ssse3")]
    {
        // ssse3_encode_full_groups_into processes 2 groups (10 src bytes → 16 dst bytes)
        // per iteration and returns the number of dst bytes written.
        let written = unsafe { ssse3_encode_full_groups_into(config, dst, src) };
        // Each 16 output bytes correspond to 10 input bytes (2 groups × 5 bytes).
        src_offset += written / 8 * 5;
        dst_offset += written;
    }

    let alphabet_ptr = config.alphabet.as_ptr();

    unsafe {
        for chunk in src[src_offset..].chunks_exact(5) {
            let b0 = chunk[0] as u32;
            let b1 = chunk[1] as u32;
            let b2 = chunk[2] as u32;
            let b3 = chunk[3] as u32;
            let b4 = chunk[4] as u32;

            let c0 = ((b0 >> 3) & 0x1F) as usize;
            let c1 = (((b0 << 2) | (b1 >> 6)) & 0x1F) as usize;
            let c2 = ((b1 >> 1) & 0x1F) as usize;
            let c3 = (((b1 << 4) | (b2 >> 4)) & 0x1F) as usize;
            let c4 = (((b2 << 1) | (b3 >> 7)) & 0x1F) as usize;
            let c5 = ((b3 >> 2) & 0x1F) as usize;
            let c6 = (((b3 << 3) | (b4 >> 5)) & 0x1F) as usize;
            let c7 = (b4 & 0x1F) as usize;

            let ptr = dst.as_mut_ptr().add(dst_offset);
            ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
            ptr.offset(1)
                .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
            ptr.offset(2)
                .write(ptr::read_unaligned(alphabet_ptr.add(c2)));
            ptr.offset(3)
                .write(ptr::read_unaligned(alphabet_ptr.add(c3)));
            ptr.offset(4)
                .write(ptr::read_unaligned(alphabet_ptr.add(c4)));
            ptr.offset(5)
                .write(ptr::read_unaligned(alphabet_ptr.add(c5)));
            ptr.offset(6)
                .write(ptr::read_unaligned(alphabet_ptr.add(c6)));
            ptr.offset(7)
                .write(ptr::read_unaligned(alphabet_ptr.add(c7)));

            dst_offset += 8;
        }

        Ok(output_len)
    }
}
