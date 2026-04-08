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
