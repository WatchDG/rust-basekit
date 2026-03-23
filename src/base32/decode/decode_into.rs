use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;

#[inline(always)]
unsafe fn decode_full_groups_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    full_groups: usize,
) -> Result<usize, Base32Error> {
    if full_groups == 0 {
        return Ok(0);
    }

    let decode_table = config.decode_table;
    let mut dst_offset = 0usize;

    for group_idx in 0..full_groups {
        let chunk_start = group_idx * 8;
        let chunk = &src[chunk_start..chunk_start + 8];

        let c0 = chunk[0];
        let c1 = chunk[1];
        let c2 = chunk[2];
        let c3 = chunk[3];
        let c4 = chunk[4];
        let c5 = chunk[5];
        let c6 = chunk[6];
        let c7 = chunk[7];

        if c0 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c1 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c2 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c3 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c4 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c5 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c6 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }
        if c7 == config.padding {
            return Err(Base32Error::InvalidPadding);
        }

        let invalid_high_bit = (c0 | c1 | c2 | c3 | c4 | c5 | c6 | c7) & 0x80;
        if invalid_high_bit != 0 {
            let pos = if c0 & 0x80 != 0 {
                0
            } else if c1 & 0x80 != 0 {
                1
            } else if c2 & 0x80 != 0 {
                2
            } else if c3 & 0x80 != 0 {
                3
            } else if c4 & 0x80 != 0 {
                4
            } else if c5 & 0x80 != 0 {
                5
            } else if c6 & 0x80 != 0 {
                6
            } else {
                7
            };
            return Err(Base32Error::InvalidCharacter(chunk[pos], chunk_start + pos));
        }

        let i0 = decode_table[c0 as usize];
        let i1 = decode_table[c1 as usize];
        let i2 = decode_table[c2 as usize];
        let i3 = decode_table[c3 as usize];
        let i4 = decode_table[c4 as usize];
        let i5 = decode_table[c5 as usize];
        let i6 = decode_table[c6 as usize];
        let i7 = decode_table[c7 as usize];

        if i0 < 0 || i1 < 0 || i2 < 0 || i3 < 0 || i4 < 0 || i5 < 0 || i6 < 0 || i7 < 0 {
            let pos = if i0 < 0 {
                0
            } else if i1 < 0 {
                1
            } else if i2 < 0 {
                2
            } else if i3 < 0 {
                3
            } else if i4 < 0 {
                4
            } else if i5 < 0 {
                5
            } else if i6 < 0 {
                6
            } else {
                7
            };
            let byte = chunk[pos];
            return Err(Base32Error::InvalidCharacter(byte, chunk_start + pos));
        }

        let b0 = ((i0 as u32) << 3) | ((i1 as u32) >> 2);
        let b1 = ((i1 as u32) << 6) | ((i2 as u32) << 1) | ((i3 as u32) >> 4);
        let b2 = ((i3 as u32) << 4) | ((i4 as u32) >> 1);
        let b3 = ((i4 as u32) << 7) | ((i5 as u32) << 2) | ((i6 as u32) >> 3);
        let b4 = ((i6 as u32) << 5) | (i7 as u32);

        unsafe {
            let ptr = dst.as_mut_ptr().add(dst_offset);
            ptr.write(b0 as u8);
            ptr.offset(1).write(b1 as u8);
            ptr.offset(2).write(b2 as u8);
            ptr.offset(3).write(b3 as u8);
            ptr.offset(4).write(b4 as u8);
        }

        dst_offset += 5;
    }

    Ok(dst_offset)
}

#[inline(always)]
unsafe fn decode_tail_into(
    config: &Base32DecodeConfig,
    chunk: &[u8],
    chunk_start: usize,
    padding_count: usize,
    dst: &mut [u8],
    dst_offset: usize,
) -> Result<usize, Base32Error> {
    let decode_table = config.decode_table;
    let mut indices: [i8; 8] = [0; 8];

    for (j, &byte) in chunk.iter().enumerate() {
        let pos = chunk_start + j;

        if byte == config.padding {
            let expected_min_pos = 8 - padding_count;
            if j < expected_min_pos {
                return Err(Base32Error::InvalidPadding);
            }
            indices[j] = 0;
        } else {
            if byte >= 128 {
                return Err(Base32Error::InvalidCharacter(byte, pos));
            }
            let val = decode_table[byte as usize];
            if val < 0 {
                return Err(Base32Error::InvalidCharacter(byte, pos));
            }
            indices[j] = val;
        }
    }

    let bytes_written = match padding_count {
        6 => 1,
        4 => 2,
        3 => 3,
        1 => 4,
        0 => 5,
        _ => return Err(Base32Error::InvalidPadding),
    };

    let i0 = indices[0] as u32;
    let i1 = indices[1] as u32;
    let i2 = indices[2] as u32;
    let i3 = indices[3] as u32;
    let i4 = indices[4] as u32;
    let i5 = indices[5] as u32;
    let i6 = indices[6] as u32;
    let i7 = indices[7] as u32;

    let b0 = (i0 << 3) | (i1 >> 2);
    let b1 = (i1 << 6) | (i2 << 1) | (i3 >> 4);
    let b2 = (i3 << 4) | (i4 >> 1);
    let b3 = (i4 << 7) | (i5 << 2) | (i6 >> 3);
    let b4 = (i6 << 5) | i7;

    unsafe {
        let ptr = dst.as_mut_ptr().add(dst_offset);

        match padding_count {
            6 => {
                ptr.write(b0 as u8);
            }
            4 => {
                ptr.write(b0 as u8);
                ptr.offset(1).write(b1 as u8);
            }
            3 => {
                ptr.write(b0 as u8);
                ptr.offset(1).write(b1 as u8);
                ptr.offset(2).write(b2 as u8);
            }
            1 => {
                ptr.write(b0 as u8);
                ptr.offset(1).write(b1 as u8);
                ptr.offset(2).write(b2 as u8);
                ptr.offset(3).write(b3 as u8);
            }
            0 => {
                ptr.write(b0 as u8);
                ptr.offset(1).write(b1 as u8);
                ptr.offset(2).write(b2 as u8);
                ptr.offset(3).write(b3 as u8);
                ptr.offset(4).write(b4 as u8);
            }
            _ => unreachable!(),
        }
    }

    Ok(bytes_written)
}

#[inline(always)]
pub fn decode_into(
    config: &Base32DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let mut padding_count = 0;
    for &byte in src.iter().rev().take(7) {
        if byte == config.padding {
            padding_count += 1;
        } else {
            break;
        }
    }

    if padding_count > 6 {
        return Err(Base32Error::InvalidPadding);
    }

    let clean_len = src.len() - padding_count;

    if clean_len == 0 {
        return Ok(0);
    }

    let output_len = (clean_len * 5) / 8;
    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let total_groups = src.len().div_ceil(8);
    let full_groups = clean_len / 8;
    let has_tail = total_groups > full_groups;

    unsafe {
        let mut dst_offset = 0usize;

        dst_offset += decode_full_groups_into(config, dst, src, full_groups)?;

        if has_tail {
            let i = full_groups * 8;
            let chunk = &src[i..core::cmp::min(i + 8, src.len())];
            dst_offset += decode_tail_into(config, chunk, i, padding_count, dst, dst_offset)?;
        }

        Ok(dst_offset)
    }
}
