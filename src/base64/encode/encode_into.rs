use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

pub(crate) use super::encode_full_groups_into;
pub(crate) use super::encode_tail_into;

#[inline]
pub fn encode_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups = src.len() / 3;
    let remainder = src.len() % 3;
    let tail_output_len = match (remainder, config.padding.is_some()) {
        (0, _) => 0,
        (1, true) => 4,
        (1, false) => 2,
        (2, true) => 4,
        (2, false) => 3,
        _ => unreachable!(),
    };
    let output_len = full_groups * 4 + tail_output_len;

    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut dst_offset = 0usize;

    dst_offset +=
        encode_full_groups_into(config, &mut dst[..full_groups * 4], &src[..full_groups * 3])?;

    if remainder > 0 {
        dst_offset += encode_tail_into(
            config,
            &mut dst[dst_offset..dst_offset + tail_output_len],
            &src[full_groups * 3..],
        )?;
    }

    Ok(dst_offset)
}
