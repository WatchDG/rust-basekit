use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;
use super::decode_into::decode_into;
use super::output::Base16DecodeOutput;

pub fn decode(
    config: &Base16DecodeConfig,
    data: impl AsRef<[u8]>,
) -> Result<Base16DecodeOutput, Base16Error> {
    let data = data.as_ref();
    if data.is_empty() {
        return Ok(Base16DecodeOutput { inner: Vec::new() });
    }

    if !data.len().is_multiple_of(2) {
        return Err(Base16Error::InvalidLength(data.len()));
    }

    let output_len = data.len() / 2;

    let mut output = Vec::with_capacity(output_len);
    unsafe { output.set_len(output_len) };

    let _ = decode_into(config, &mut output, data)?;
    Ok(Base16DecodeOutput { inner: output })
}
