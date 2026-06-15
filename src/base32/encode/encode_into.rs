use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

pub(crate) use super::encode_full_groups_into;
pub(crate) use super::encode_tail_into;

#[inline]
pub fn encode32_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups_count = src.len() / 5;
    let remainder = src.len() % 5;

    let tail_output_len = match (remainder, config.padding.is_some()) {
        (0, _) => 0,
        (1, true) => 8,
        (1, false) => 2,
        (2, true) => 8,
        (2, false) => 4,
        (3, true) => 8,
        (3, false) => 5,
        (4, true) => 8,
        (4, false) => 7,
        _ => unreachable!(),
    };

    let output_len = full_groups_count * 8 + tail_output_len;

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let full_src = &src[..full_groups_count * 5];
    let tail_src = if tail_output_len > 0 {
        Some(&src[full_groups_count * 5..])
    } else {
        None
    };

    let mut offset = 0usize;

    if !full_src.is_empty() {
        offset += encode_full_groups_into(config, &mut dst[offset..], full_src)?;
    }

    if let Some(tail) = tail_src {
        offset += unsafe { encode_tail_into(config, &mut dst[offset..], tail)? };
    }

    Ok(offset)
}
