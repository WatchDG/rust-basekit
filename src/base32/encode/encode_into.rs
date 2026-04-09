use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

pub(crate) use super::encode_full_groups_into;
pub(crate) use super::encode_tail_into;

#[inline]
pub fn encode_into(
    config: &Base32EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base32Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups_count = src.len() / 5;
    let remainder = src.len() % 5;
    let output_len = full_groups_count * 8
        + match remainder {
            0 => 0,
            1 => 8,
            2 => 8,
            3 => 8,
            4 => 8,
            _ => unreachable!(),
        };

    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let full_src = &src[..full_groups_count * 5];
    let tail_src = if remainder > 0 {
        Some(&src[full_groups_count * 5..])
    } else {
        None
    };

    let mut offset = 0usize;

    if !full_src.is_empty() {
        offset += encode_full_groups_into(config, &mut dst[offset..], full_src)?;
    }

    if let Some(tail) = tail_src {
        encode_tail_into(config, &mut dst[offset..], tail)?;
    }

    Ok(output_len)
}
