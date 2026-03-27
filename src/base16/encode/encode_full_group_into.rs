use core::ptr;

use super::super::config::Base16EncodeConfig;
use super::super::error::Base16Error;

#[inline(always)]
pub(crate) fn encode_full_group_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    let alphabet_ptr = config.alphabet.as_ptr();
    let mut src_offset = 0usize;
    let mut dst_offset = 0usize;

    while src_offset < src.len() {
        if dst_offset + 2 > dst.len() {
            return Err(Base16Error::DestinationBufferTooSmall {
                needed: src.len() * 2,
                provided: dst.len(),
            });
        }

        let byte = src[src_offset];
        let ptr = dst.as_mut_ptr();

        unsafe {
            ptr.add(dst_offset)
                .write(ptr::read_unaligned(alphabet_ptr.add((byte >> 4) as usize)));
            ptr.add(dst_offset + 1).write(ptr::read_unaligned(
                alphabet_ptr.add((byte & 0x0F) as usize),
            ));
        }

        src_offset += 1;
        dst_offset += 2;
    }

    Ok(dst_offset)
}
