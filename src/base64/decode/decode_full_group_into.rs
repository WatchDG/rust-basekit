use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

#[inline(always)]
#[allow(unsafe_op_in_unsafe_fn)]
pub(crate) unsafe fn decode_full_group_into(
    config: &Base64DecodeConfig,
    chunk: &[u8],
    chunk_start: usize,
    dst: &mut [u8],
    dst_offset: usize,
) -> Result<usize, Base64Error> {
    debug_assert_eq!(chunk.len(), 4);

    let b0 = chunk[0];
    let b1 = chunk[1];
    let b2 = chunk[2];
    let b3 = chunk[3];

    if b0 == config.padding || b1 == config.padding || b2 == config.padding || b3 == config.padding
    {
        return Err(Base64Error::InvalidPadding);
    }

    if (b0 | b1 | b2 | b3) & 0x80 != 0 {
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

    let decode_table = config.decode_table;
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
        return Err(Base64Error::InvalidCharacter(chunk[pos], chunk_start + pos));
    }

    let triple = ((i0 as u32) << 18) | ((i1 as u32) << 12) | ((i2 as u32) << 6) | (i3 as u32);

    let ptr = dst.as_mut_ptr().add(dst_offset);
    ptr.write((triple >> 16) as u8);
    ptr.add(1).write((triple >> 8) as u8);
    ptr.add(2).write(triple as u8);

    Ok(3)
}
