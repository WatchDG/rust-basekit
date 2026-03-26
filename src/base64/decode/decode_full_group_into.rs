use crate::base64::config::Base64DecodeConfig;
use crate::base64::error::Base64Error;

#[inline(always)]
pub(crate) fn decode_full_group_into(
    config: &Base64DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
    src_offset: usize,
) -> Result<usize, Base64Error> {
    debug_assert_eq!(src.len(), 4);

    let b0 = src[0];
    let b1 = src[1];
    let b2 = src[2];
    let b3 = src[3];

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
        return Err(Base64Error::InvalidCharacter(src[pos], src_offset + pos));
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
        return Err(Base64Error::InvalidCharacter(src[pos], src_offset + pos));
    }

    let triple = ((i0 as u32) << 18) | ((i1 as u32) << 12) | ((i2 as u32) << 6) | (i3 as u32);

    dst[0] = (triple >> 16) as u8;
    dst[1] = (triple >> 8) as u8;
    dst[2] = triple as u8;

    Ok(3)
}
