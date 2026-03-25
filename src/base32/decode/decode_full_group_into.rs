use crate::base32::config::Base32DecodeConfig;
use crate::base32::error::Base32Error;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn decode_full_group_into(
    config: &Base32DecodeConfig,
    chunk: &[u8],
    chunk_start: usize,
    dst: &mut [u8],
    dst_offset: usize,
) -> Result<usize, Base32Error> {
    debug_assert_eq!(chunk.len(), 8);

    let c0 = chunk[0];
    let c1 = chunk[1];
    let c2 = chunk[2];
    let c3 = chunk[3];
    let c4 = chunk[4];
    let c5 = chunk[5];
    let c6 = chunk[6];
    let c7 = chunk[7];

    if c0 == config.padding
        || c1 == config.padding
        || c2 == config.padding
        || c3 == config.padding
        || c4 == config.padding
        || c5 == config.padding
        || c6 == config.padding
        || c7 == config.padding
    {
        return Err(Base32Error::InvalidPadding);
    }

    if (c0 | c1 | c2 | c3 | c4 | c5 | c6 | c7) & 0x80 != 0 {
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

    let decode_table = config.decode_table;
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
        return Err(Base32Error::InvalidCharacter(chunk[pos], chunk_start + pos));
    }

    let b0 = ((i0 as u32) << 3) | ((i1 as u32) >> 2);
    let b1 = ((i1 as u32) << 6) | ((i2 as u32) << 1) | ((i3 as u32) >> 4);
    let b2 = ((i3 as u32) << 4) | ((i4 as u32) >> 1);
    let b3 = ((i4 as u32) << 7) | ((i5 as u32) << 2) | ((i6 as u32) >> 3);
    let b4 = ((i6 as u32) << 5) | (i7 as u32);

    let ptr = dst.as_mut_ptr().add(dst_offset);
    ptr.write(b0 as u8);
    ptr.add(1).write(b1 as u8);
    ptr.add(2).write(b2 as u8);
    ptr.add(3).write(b3 as u8);
    ptr.add(4).write(b4 as u8);

    Ok(5)
}
