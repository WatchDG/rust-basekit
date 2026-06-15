use core::ptr;

use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn encode_tail_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    debug_assert!(
        src.len() == 1 || src.len() == 2,
        "src length must be 1 or 2"
    );

    let output_len = match (src.len(), config.padding.is_some()) {
        (1, true) => 4,
        (1, false) => 2,
        (2, true) => 4,
        (2, false) => 3,
        _ => unreachable!(),
    };

    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let alphabet_ptr = config.alphabet.as_ptr();
    let ptr = dst.as_mut_ptr();

    match src.len() {
        1 => {
            let triple = (src[0] as u32) << 16;
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;

            ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
            ptr.offset(1)
                .write(ptr::read_unaligned(alphabet_ptr.add(c1)));

            if let Some(padding) = config.padding {
                ptr.offset(2).write(padding);
                ptr.offset(3).write(padding);
                Ok(4)
            } else {
                Ok(2)
            }
        }
        2 => {
            let triple = ((src[0] as u32) << 16) | ((src[1] as u32) << 8);
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;
            let c2 = ((triple >> 6) & 0x3F) as usize;

            ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
            ptr.offset(1)
                .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
            ptr.offset(2)
                .write(ptr::read_unaligned(alphabet_ptr.add(c2)));

            if let Some(padding) = config.padding {
                ptr.offset(3).write(padding);
                Ok(4)
            } else {
                Ok(3)
            }
        }
        0 => Ok(0),
        _ => unreachable!(),
    }
}
