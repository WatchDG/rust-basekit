use core::ptr;

use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

#[inline(always)]
pub fn encode_tail_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    debug_assert!(
        !src.is_empty() && src.len() <= 4,
        "src length must be between 1 and 4"
    );

    let src_len = src.len();
    let output_len_with_padding = 8;
    let output_len_without_padding = match src_len {
        1 => 2,
        2 => 4,
        3 => 5,
        4 => 7,
        _ => unreachable!(),
    };
    let output_len = output_len_with_padding;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let alphabet_ptr = config.alphabet.as_ptr();

    unsafe {
        let ptr = dst.as_mut_ptr();

        match src_len {
            1 => {
                let b0 = src[0] as u32;
                let c0 = ((b0 >> 3) & 0x1F) as usize;
                let c1 = ((b0 << 2) & 0x1F) as usize;

                ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
                ptr.offset(1)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c1)));

                if let Some(padding) = config.padding {
                    ptr.offset(2).write(padding);
                    ptr.offset(3).write(padding);
                    ptr.offset(4).write(padding);
                    ptr.offset(5).write(padding);
                    ptr.offset(6).write(padding);
                    ptr.offset(7).write(padding);
                    Ok(output_len)
                } else {
                    Ok(output_len_without_padding)
                }
            }
            2 => {
                let b0 = src[0] as u32;
                let b1 = src[1] as u32;
                let c0 = ((b0 >> 3) & 0x1F) as usize;
                let c1 = (((b0 << 2) | (b1 >> 6)) & 0x1F) as usize;
                let c2 = ((b1 >> 1) & 0x1F) as usize;
                let c3 = ((b1 << 4) & 0x1F) as usize;

                ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
                ptr.offset(1)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
                ptr.offset(2)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c2)));
                ptr.offset(3)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c3)));

                if let Some(padding) = config.padding {
                    ptr.offset(4).write(padding);
                    ptr.offset(5).write(padding);
                    ptr.offset(6).write(padding);
                    ptr.offset(7).write(padding);
                    Ok(output_len)
                } else {
                    Ok(output_len_without_padding)
                }
            }
            3 => {
                let b0 = src[0] as u32;
                let b1 = src[1] as u32;
                let b2 = src[2] as u32;
                let c0 = ((b0 >> 3) & 0x1F) as usize;
                let c1 = (((b0 << 2) | (b1 >> 6)) & 0x1F) as usize;
                let c2 = ((b1 >> 1) & 0x1F) as usize;
                let c3 = (((b1 << 4) | (b2 >> 4)) & 0x1F) as usize;
                let c4 = ((b2 << 1) & 0x1F) as usize;

                ptr.write(ptr::read_unaligned(alphabet_ptr.add(c0)));
                ptr.offset(1)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c1)));
                ptr.offset(2)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c2)));
                ptr.offset(3)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c3)));
                ptr.offset(4)
                    .write(ptr::read_unaligned(alphabet_ptr.add(c4)));

                if let Some(padding) = config.padding {
                    ptr.offset(5).write(padding);
                    ptr.offset(6).write(padding);
                    ptr.offset(7).write(padding);
                    Ok(output_len)
                } else {
                    Ok(output_len_without_padding)
                }
            }
            4 => {
                let b0 = src[0] as u32;
                let b1 = src[1] as u32;
                let b2 = src[2] as u32;
                let b3 = src[3] as u32;
                let c0 = ((b0 >> 3) & 0x1F) as usize;
                let c1 = (((b0 << 2) | (b1 >> 6)) & 0x1F) as usize;
                let c2 = ((b1 >> 1) & 0x1F) as usize;
                let c3 = (((b1 << 4) | (b2 >> 4)) & 0x1F) as usize;
                let c4 = (((b2 << 1) | (b3 >> 7)) & 0x1F) as usize;
                let c5 = ((b3 >> 2) & 0x1F) as usize;
                let c6 = ((b3 << 3) & 0x1F) as usize;

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

                if let Some(padding) = config.padding {
                    ptr.offset(7).write(padding);
                    Ok(output_len)
                } else {
                    Ok(output_len_without_padding)
                }
            }
            _ => unreachable!(),
        }
    }
}
