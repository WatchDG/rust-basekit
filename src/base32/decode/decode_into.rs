use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;
use super::decode_full_groups_into::decode_full_groups_into;
use super::decode_tail_into::decode_tail_into;

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

    let mut dst_offset = 0usize;

    dst_offset += decode_full_groups_into(config, dst, src, full_groups)?;

    if has_tail {
        let tail_src = &src[full_groups * 8..core::cmp::min(full_groups * 8 + 8, src.len())];
        dst_offset += decode_tail_into(
            config,
            &mut dst[dst_offset..],
            tail_src,
            full_groups * 8,
            padding_count,
        )?;
    }

    Ok(dst_offset)
}
