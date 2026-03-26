use core::ptr;

use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

#[inline(always)]
pub fn encode_full_group_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    debug_assert_eq!(src.len(), 3, "src length must be exactly 3");
    debug_assert!(dst.len() >= 4, "dst must have at least 4 bytes");

    let alphabet_ptr = config.alphabet.as_ptr();
    let triple = ((src[0] as u32) << 16) | ((src[1] as u32) << 8) | (src[2] as u32);
    let ptr = dst.as_mut_ptr();

    unsafe {
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
    }

    Ok(4)
}
