use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;
use super::encode_into_slice::encode_into_slice;

#[inline]
pub fn encode_into(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base64Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups_count = src.len() / 3;
    let remainder = src.len() % 3;

    let tail_output_len = match (remainder, config.padding.is_some()) {
        (0, _) => 0,
        (1, true) => 4,
        (1, false) => 2,
        (2, true) => 4,
        (2, false) => 3,
        _ => unreachable!(),
    };

    let output_len = full_groups_count * 4 + tail_output_len;

    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let full_groups_src = if full_groups_count > 0 {
        Some(&src[..full_groups_count * 3])
    } else {
        None
    };

    let tail_src = if tail_output_len > 0 {
        Some(&src[full_groups_count * 3..])
    } else {
        None
    };

    encode_into_slice(config, dst, full_groups_src, tail_src)
}
