use core::ptr;

use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

#[inline(always)]
pub(crate) fn encode_full_group_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert_eq!(src.len() % 5, 0, "src length must be a multiple of 5");

    if dst.len() < 8 {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: 8,
            provided: dst.len(),
        });
    }

    let b0 = src[0] as u32;
    let b1 = src[1] as u32;
    let b2 = src[2] as u32;
    let b3 = src[3] as u32;
    let b4 = src[4] as u32;

    let c0 = ((b0 >> 3) & 0x1F) as usize;
    let c1 = (((b0 << 2) | (b1 >> 6)) & 0x1F) as usize;
    let c2 = ((b1 >> 1) & 0x1F) as usize;
    let c3 = (((b1 << 4) | (b2 >> 4)) & 0x1F) as usize;
    let c4 = (((b2 << 1) | (b3 >> 7)) & 0x1F) as usize;
    let c5 = ((b3 >> 2) & 0x1F) as usize;
    let c6 = (((b3 << 3) | (b4 >> 5)) & 0x1F) as usize;
    let c7 = (b4 & 0x1F) as usize;

    let alphabet_ptr = config.alphabet.as_ptr();
    let ptr = dst.as_mut_ptr();

    unsafe {
        ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
        ptr.add(1).write(ptr::read_unaligned(alphabet_ptr.add(c1)));
        ptr.add(2).write(ptr::read_unaligned(alphabet_ptr.add(c2)));
        ptr.add(3).write(ptr::read_unaligned(alphabet_ptr.add(c3)));
        ptr.add(4).write(ptr::read_unaligned(alphabet_ptr.add(c4)));
        ptr.add(5).write(ptr::read_unaligned(alphabet_ptr.add(c5)));
        ptr.add(6).write(ptr::read_unaligned(alphabet_ptr.add(c6)));
        ptr.add(7).write(ptr::read_unaligned(alphabet_ptr.add(c7)));
    }

    Ok(8)
}
