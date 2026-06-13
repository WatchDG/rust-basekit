use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

#[inline(always)]
pub(crate) unsafe fn decode_tail_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    dst_offset: usize,
    tail: &[u8],
    src_offset: usize,
    padding_count: usize,
) -> Result<usize, Base64Error> {
    let tail_len = tail.len();

    if tail_len == 0 {
        return Ok(0);
    }

    let bytes_written = match padding_count {
        2 => 1,
        1 => 2,
        0 => match tail_len {
            1 => 0,
            2 => 1,
            3 => 2,
            _ => panic!("invalid unpadded base64 tail length: {}", tail_len),
        },
        _ => panic!("invalid base64 padding count: {}", padding_count),
    };

    let decode_table = config.decode_table;
    let mut indices: [i8; 4] = [0; 4];
    for (j, &byte) in tail.iter().enumerate() {
        let pos = src_offset + j;

        if config.padding == Some(byte) {
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

    let three_bytes = ((indices[0] as u32) << 18)
        | ((indices[1] as u32) << 12)
        | ((indices[2] as u32) << 6)
        | (indices[3] as u32);

    unsafe {
        let ptr = dst.as_mut_ptr().add(dst_offset);

        match bytes_written {
            0 => {}
            1 => {
                ptr.write((three_bytes >> 16) as u8);
            }
            2 => {
                ptr.write((three_bytes >> 16) as u8);
                ptr.offset(1).write((three_bytes >> 8) as u8);
            }
            3 => {
                ptr.write((three_bytes >> 16) as u8);
                ptr.offset(1).write((three_bytes >> 8) as u8);
                ptr.offset(2).write(three_bytes as u8);
            }
            _ => panic!("invalid bytes_written value: {}", bytes_written),
        }
    }

    Ok(bytes_written)
}
