use super::super::config::Base64Config;
use super::super::consts::DECODE_TABLE;
use super::super::error::Base64Error;

pub fn decode_v1(config: &Base64Config, data: &[u8]) -> Result<Vec<u8>, Base64Error> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut padding_count = 0;
    for &byte in data.iter().rev() {
        if byte == config.padding {
            padding_count += 1;
        } else {
            break;
        }
    }

    if padding_count > 2 {
        return Err(Base64Error::InvalidPadding);
    }

    let clean_len = data.len() - padding_count;

    if clean_len == 0 {
        return Ok(Vec::new());
    }

    let output_len = (clean_len * 3) / 4;
    let mut output = Vec::with_capacity(output_len);
    let total_groups = data.len().div_ceil(4);

    for group_idx in 0..total_groups {
        let i = group_idx * 4;
        let chunk = &data[i..std::cmp::min(i + 4, data.len())];

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
                let val = DECODE_TABLE[byte as usize];
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

        if bytes_to_add >= 1 {
            output.push((triple >> 16) as u8);
        }
        if bytes_to_add >= 2 {
            output.push((triple >> 8) as u8);
        }
        if bytes_to_add >= 3 {
            output.push(triple as u8);
        }
    }

    Ok(output)
}
