use core::ptr;

use super::super::config::Base16EncodeConfig;
use super::super::error::Base16Error;

#[inline(always)]
pub fn encode_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let output_len = src.len() * 2;

    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let alphabet_ptr = config.alphabet.as_ptr();

    unsafe {
        let mut src_offset = 0usize;
        let mut dst_offset = 0usize;

        while src_offset < src.len() {
            let byte = src[src_offset];
            let ptr = dst.as_mut_ptr().add(dst_offset);

            ptr.write(ptr::read_unaligned(alphabet_ptr.add((byte >> 4) as usize)));
            ptr.offset(1).write(ptr::read_unaligned(
                alphabet_ptr.add((byte & 0x0F) as usize),
            ));

            src_offset += 1;
            dst_offset += 2;
        }

        Ok(output_len)
    }
}
