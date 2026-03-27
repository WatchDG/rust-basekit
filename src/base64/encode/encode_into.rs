use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

pub(crate) use super::encode_full_groups_into;
pub use super::encode_tail_into;

#[inline(always)]
pub fn encode_into(
    config: &Base64EncodeConfig,
    mut dst: impl AsMut<[u8]>,
    src: impl AsRef<[u8]>,
) -> Result<usize, Base64Error> {
    let src = src.as_ref();
    if src.is_empty() {
        return Ok(0);
    }

    let full_groups = src.len() / 3;
    let tail_len = src.len() % 3;
    let output_len = full_groups * 4
        + match tail_len {
            0 => 0,
            1 | 2 => 4,
            _ => unreachable!(),
        };

    let dst = dst.as_mut();
    if dst.len() < output_len {
        return Err(Base64Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let mut dst_offset = 0usize;

    dst_offset +=
        encode_full_groups_into(config, &mut dst[..full_groups * 4], &src[..full_groups * 3])?;

    if tail_len > 0 {
        dst_offset +=
            encode_tail_into(config, &mut dst[dst_offset..][..4], &src[full_groups * 3..])?;
    }

    Ok(dst_offset)
}
