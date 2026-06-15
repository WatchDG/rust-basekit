use super::super::config::Base16EncodeConfig;
use super::super::error::Base16Error;
use super::encode_full_groups_into::encode_full_groups_into;

#[inline]
pub fn encode16_into(
    config: &Base16EncodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    if src.is_empty() {
        return Ok(0);
    }

    let output_len = src.len() * 2;

    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    encode_full_groups_into(config, dst, src)
}
