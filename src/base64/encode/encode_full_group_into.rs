use core::ptr;

use super::super::config::Base64EncodeConfig;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub unsafe fn encode_full_group_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    debug_assert!(src.len() == 3);
    debug_assert!(dst.len() >= 4);

    let alphabet_ptr = config.alphabet.as_ptr();
    let triple = ((src[0] as u32) << 16) | ((src[1] as u32) << 8) | (src[2] as u32);
    let ptr = dst.as_mut_ptr();

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

    4
}
