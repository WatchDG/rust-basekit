use core::ptr;

use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

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

    let alphabet_ptr = config.alphabet.as_ptr();

    unsafe {
        let mut offset = 0usize;

        for chunk in src.chunks_exact(5) {
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

            let ptr = dst.as_mut_ptr().add(offset);
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

            offset += 8;
        }

        Ok(output_len)
    }
}

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
    let output_len = 8;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let alphabet_ptr = config.alphabet.as_ptr();
    let padding = config.padding;

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
                ptr.offset(2).write(padding);
                ptr.offset(3).write(padding);
                ptr.offset(4).write(padding);
                ptr.offset(5).write(padding);
                ptr.offset(6).write(padding);
                ptr.offset(7).write(padding);
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
                ptr.offset(4).write(padding);
                ptr.offset(5).write(padding);
                ptr.offset(6).write(padding);
                ptr.offset(7).write(padding);
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
                ptr.offset(5).write(padding);
                ptr.offset(6).write(padding);
                ptr.offset(7).write(padding);
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
                ptr.offset(7).write(padding);
            }
            _ => unreachable!(),
        }

        Ok(output_len)
    }
}

#[inline(always)]
pub fn encode_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_chunks = src.len() / 5;
    let remainder = src.len() % 5;
    let output_len = full_chunks * 8
        + match remainder {
            0 => 0,
            1 => 8,
            2 => 8,
            3 => 8,
            4 => 8,
            _ => unreachable!(),
        };

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let full_src = &src[..full_chunks * 5];
    let tail_src = if remainder > 0 {
        Some(&src[full_chunks * 5..])
    } else {
        None
    };

    let mut offset = 0usize;

    if !full_src.is_empty() {
        offset += encode_full_groups_into(config, &mut dst[offset..], full_src)?;
    }

    if let Some(tail) = tail_src {
        encode_tail_into(config, &mut dst[offset..], tail)?;
    }

    Ok(output_len)
}
