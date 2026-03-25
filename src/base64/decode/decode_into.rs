use super::super::config::Base64DecodeConfig;
use super::super::error::Base64Error;
use super::decode_full_groups_into::decode_full_groups_into;
use super::decode_tail_into::decode_tail_into;

#[inline(always)]
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
