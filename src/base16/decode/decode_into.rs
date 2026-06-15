use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;
use super::decode_full_groups_into::decode_full_groups_into;

#[inline]
pub fn decode16_into(
    config: &Base16DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    if src.is_empty() {
        return Ok(0);
    }

    if !src.len().is_multiple_of(2) {
        return Err(Base16Error::InvalidLength(src.len()));
    }

    let output_len = src.len() / 2;

    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    decode_full_groups_into(config, dst, src)
}
