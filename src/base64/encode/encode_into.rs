use core::ptr;

use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

#[inline(always)]
pub fn encode_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_chunks = src.len() / 3;
    let remainder = src.len() % 3;
    let output_len = full_chunks * 4
        + match remainder {
            0 => 0,
            1 => 4,
            2 => 4,
            _ => unreachable!(),
        };

    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let alphabet_ptr = config.alphabet.as_ptr();
    let padding = config.padding;

    unsafe {
        let mut offset = 0usize;

        for chunk in src[..full_chunks * 3].chunks_exact(3) {
            let triple = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
            let ptr = dst.as_mut_ptr().add(offset);

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

            offset += 4;
        }

        match remainder {
            1 => {
                let triple = (src[src.len() - 1] as u32) << 16;
                let c0 = ((triple >> 18) & 0x3F) as usize;
                let c1 = ((triple >> 12) & 0x3F) as usize;
                let ptr = dst.as_mut_ptr().add(offset);

                ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
                ptr.offset(1)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
                ptr.offset(2).write(padding);
                ptr.offset(3).write(padding);
            }
            2 => {
                let triple =
                    ((src[src.len() - 2] as u32) << 16) | ((src[src.len() - 1] as u32) << 8);
                let c0 = ((triple >> 18) & 0x3F) as usize;
                let c1 = ((triple >> 12) & 0x3F) as usize;
                let c2 = ((triple >> 6) & 0x3F) as usize;
                let ptr = dst.as_mut_ptr().add(offset);

                ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
                ptr.offset(1)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
                ptr.offset(2)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c2)));
                ptr.offset(3).write(padding);
            }
            0 => {}
            _ => unreachable!(),
        }

        Ok(output_len)
    }
}
