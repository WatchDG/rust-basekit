use super::super::config::Base32EncodeConfig;
use super::super::error::Base32Error;

pub(crate) use super::encode_full_groups_into::encode_full_groups_into;
pub use super::encode_tail_into::encode_tail_into;

#[inline(always)]
pub fn encode_into(
    config: &Base32EncodeConfig,
    mut dst: impl AsMut<[u8]>,
    src: impl AsRef<[u8]>,
) -> Result<usize, Base32Error> {
    let src = src.as_ref();
    if src.is_empty() {
        return Ok(0);
    }

    let full_chunks = src.len() / 5;
    let remainder = src.len() % 5;
    let output_len = full_chunks * 8
        + match remainder {
            0 => 0,
            1 => 8,
            2 => 8,
            3 => 8,
            4 => 8,
            _ => unreachable!(),
        };

    let dst = dst.as_mut();
    if dst.len() < output_len {
        return Err(Base32Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let full_src = &src[..full_chunks * 5];
    let tail_src = if remainder > 0 {
        Some(&src[full_chunks * 5..])
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
