use super::super::config::Base64DecodeConfig;
use super::super::error::Base64Error;

unsafe fn decode_full_groups_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    full_groups: usize,
) -> Result<usize, Base64Error> {
    if full_groups == 0 {
        return Ok(0);
    }

    let decode_table = config.decode_table;
    let mut dst_offset = 0usize;

    for group_idx in 0..full_groups {
        let chunk_start = group_idx * 4;
        let chunk = &src[chunk_start..chunk_start + 4];

        let b0 = chunk[0];
        let b1 = chunk[1];
        let b2 = chunk[2];
        let b3 = chunk[3];

        if b0 == config.padding {
            return Err(Base64Error::InvalidPadding);
        }
        if b1 == config.padding {
            return Err(Base64Error::InvalidPadding);
        }
        if b2 == config.padding {
            return Err(Base64Error::InvalidPadding);
        }
        if b3 == config.padding {
            return Err(Base64Error::InvalidPadding);
        }

        let invalid_high_bit = (b0 | b1 | b2 | b3) & 0x80;
        if invalid_high_bit != 0 {
            let pos = if b0 & 0x80 != 0 {
                0
            } else if b1 & 0x80 != 0 {
                1
            } else if b2 & 0x80 != 0 {
                2
            } else {
                3
            };
            return Err(Base64Error::InvalidCharacter(chunk[pos], chunk_start + pos));
        }

        let i0 = decode_table[b0 as usize];
        let i1 = decode_table[b1 as usize];
        let i2 = decode_table[b2 as usize];
        let i3 = decode_table[b3 as usize];

        if i0 < 0 || i1 < 0 || i2 < 0 || i3 < 0 {
            let pos = if i0 < 0 {
                0
            } else if i1 < 0 {
                1
            } else if i2 < 0 {
                2
            } else {
                3
            };
            let byte = chunk[pos];
            return Err(Base64Error::InvalidCharacter(byte, chunk_start + pos));
        }

        let triple = ((i0 as u32) << 18) | ((i1 as u32) << 12) | ((i2 as u32) << 6) | (i3 as u32);

        unsafe {
            let ptr = dst.as_mut_ptr().add(dst_offset);
            ptr.write((triple >> 16) as u8);
            ptr.offset(1).write((triple >> 8) as u8);
            ptr.offset(2).write(triple as u8);
        }

        dst_offset += 3;
    }

    Ok(dst_offset)
}

unsafe fn decode_tail_into(
    config: &Base64DecodeConfig,
    chunk: &[u8],
    chunk_start: usize,
    padding_count: usize,
    dst: &mut [u8],
    dst_offset: usize,
) -> Result<usize, Base64Error> {
    let decode_table = config.decode_table;
    let mut indices: [i8; 4] = [0; 4];
    for (j, &byte) in chunk.iter().enumerate() {
        let pos = chunk_start + j;

        if byte == config.padding {
            let expected_min_pos = 4 - padding_count;
            if j < expected_min_pos {
                return Err(Base64Error::InvalidPadding);
            }
            indices[j] = 0;
        } else {
            if byte >= 128 {
                return Err(Base64Error::InvalidCharacter(byte, pos));
            }
            let val = decode_table[byte as usize];
            if val < 0 {
                return Err(Base64Error::InvalidCharacter(byte, pos));
            }
            indices[j] = val;
        }
    }

    let triple = ((indices[0] as u32) << 18)
        | ((indices[1] as u32) << 12)
        | ((indices[2] as u32) << 6)
        | (indices[3] as u32);

    let bytes_written = match padding_count {
        2 => 1,
        1 => 2,
        _ => 3,
    };

    unsafe {
        let ptr = dst.as_mut_ptr().add(dst_offset);

        match padding_count {
            2 => {
                ptr.write((triple >> 16) as u8);
            }
            1 => {
                ptr.write((triple >> 16) as u8);
                ptr.offset(1).write((triple >> 8) as u8);
            }
            _ => {
                ptr.write((triple >> 16) as u8);
                ptr.offset(1).write((triple >> 8) as u8);
                ptr.offset(2).write(triple as u8);
            }
        }
    }

    Ok(bytes_written)
}

pub fn decode_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let mut padding_count = 0;
    for &byte in src.iter().rev().take(3) {
        if byte == config.padding {
            padding_count += 1;
        } else {
            break;
        }
    }

    if padding_count > 2 {
        return Err(Base64Error::InvalidPadding);
    }

    let clean_len = src.len() - padding_count;

    if clean_len == 0 {
        return Ok(0);
    }

    let output_len = (clean_len * 3) / 4;
    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let total_groups = src.len().div_ceil(4);
    let full_groups = clean_len / 4;
    let has_tail = total_groups > full_groups;

    unsafe {
        let mut dst_offset = 0usize;

        dst_offset += decode_full_groups_into(config, dst, src, full_groups)?;

        if has_tail {
            let i = full_groups * 4;
            let chunk = &src[i..core::cmp::min(i + 4, src.len())];
            dst_offset += decode_tail_into(config, chunk, i, padding_count, dst, dst_offset)?;
        }

        Ok(dst_offset)
    }
}
