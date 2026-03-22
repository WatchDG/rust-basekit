use super::super::config::Base64DecodeConfig;
use super::super::error::Base64Error;

pub fn decode_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let mut padding_count = 0;
    for &byte in src.iter().rev() {
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
    let decode_table = config.decode_table;

    unsafe {
        let mut dst_offset = 0usize;

        for group_idx in 0..total_groups {
            let i = group_idx * 4;
            let chunk = &src[i..core::cmp::min(i + 4, src.len())];

            let mut indices: [i8; 4] = [0; 4];
            for (j, &byte) in chunk.iter().enumerate() {
                let pos = i + j;

                if byte == config.padding {
                    if pos < clean_len {
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

            let i0 = indices[0] as u32;
            let i1 = indices[1] as u32;
            let i2 = indices[2] as u32;
            let i3 = indices[3] as u32;

            let triple = (i0 << 18) | (i1 << 12) | (i2 << 6) | i3;

            let is_last_group = group_idx == total_groups - 1;

            let bytes_to_add = if is_last_group {
                match padding_count {
                    2 => 1,
                    1 => 2,
                    _ => 3,
                }
            } else {
                3
            };

            let ptr = dst.as_mut_ptr().add(dst_offset);

            if bytes_to_add >= 1 {
                ptr.write((triple >> 16) as u8);
                dst_offset += 1;
            }
            if bytes_to_add >= 2 {
                ptr.offset(1).write((triple >> 8) as u8);
                dst_offset += 1;
            }
            if bytes_to_add >= 3 {
                ptr.offset(2).write(triple as u8);
                dst_offset += 1;
            }
        }

        Ok(dst_offset)
    }
}
