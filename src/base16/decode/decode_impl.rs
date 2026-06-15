use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;
use super::decode_into::decode16_into;
use super::output::Base16DecodeOutput;
use crate::utils::init_vec_with;

#[inline]
pub fn decode16(
    config: &Base16DecodeConfig,
    data: impl AsRef<[u8]>,
) -> Result<Base16DecodeOutput, Base16Error> {
    let data = data.as_ref();

    if data.is_empty() {
        return Ok(Base16DecodeOutput { inner: Vec::new() });
    }

    if data.len() % 2 != 0 {
        return Err(Base16Error::InvalidLength(data.len()));
    }

    let output_len = data.len() / 2;

    let output = unsafe { init_vec_with(output_len, |buf| decode16_into(config, buf, data))? };

    Ok(Base16DecodeOutput { inner: output })
}
