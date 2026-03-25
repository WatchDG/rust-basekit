use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;

#[inline(always)]
pub unsafe fn decode_tail_into(
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
