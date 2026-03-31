use super::super::config::Base64EncodeConfig;
use super::super::error::Base64Error;

pub(crate) use super::encode_full_groups_into;
pub(crate) use super::encode_tail_into;

#[inline(always)]
pub(crate) fn encode_into_slice(
    config: &Base64EncodeConfig,
    dst: &mut [u8],
    full_groups_src: Option<&[u8]>,
    tail_src: Option<&[u8]>,
) -> Result<usize, Base64Error> {
    let mut dst_offset = 0usize;

    if let Some(src) = full_groups_src {
        dst_offset += encode_full_groups_into(config, dst, src)?;
    }

    if let Some(src) = tail_src {
        dst_offset += encode_tail_into(config, &mut dst[dst_offset..], src)?;
    }

    Ok(dst_offset)
}
