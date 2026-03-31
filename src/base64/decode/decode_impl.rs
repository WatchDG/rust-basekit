use super::super::config::Base64DecodeConfig;
use super::super::error::Base64Error;
use super::decode_into::decode_into;
use super::output::Base64DecodeOutput;

#[inline]
pub fn decode(
    config: &Base64DecodeConfig,
    data: impl AsRef<[u8]>,
) -> Result<Base64DecodeOutput, Base64Error> {
    let data = data.as_ref();

    if data.is_empty() {
        return Ok(Base64DecodeOutput { inner: Vec::new() });
    }

    let mut padding_count = 0;
    for &byte in data.iter().rev() {
        if config.padding == Some(byte) {
            padding_count += 1;
        } else {
            break;
        }
    }

    if padding_count > 2 {
        return Err(Base64Error::InvalidPadding);
    }

    let clean_len = data.len() - padding_count;

    if clean_len == 0 {
        return Ok(Base64DecodeOutput { inner: Vec::new() });
    }

    let output_len = (clean_len * 3) / 4;

    let mut output = Vec::with_capacity(output_len);
    unsafe { output.set_len(output_len) };

    let _ = decode_into(config, &mut output, data)?;

    Ok(Base64DecodeOutput { inner: output })
}
