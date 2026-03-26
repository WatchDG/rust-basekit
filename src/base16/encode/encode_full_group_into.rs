use core::ptr;

use super::super::config::Base16EncodeConfig;

#[inline(always)]
pub unsafe fn encode_full_group_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> usize {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset < src.len() {
        let byte = src[src_offset];
        let ptr = dst.as_mut_ptr().add(dst_offset);

        unsafe {
            ptr.write(ptr::read_unaligned(alphabet_ptr.add((byte >> 4) as usize)));
            ptr.offset(1).write(ptr::read_unaligned(
                alphabet_ptr.add((byte & 0x0F) as usize),
            ));
        }

        src_offset += 1;
        dst_offset += 2;
    }

    dst_offset
}
