use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;
use super::decode_into::decode_into;

pub fn decode(config: &Base16DecodeConfig, data: &[u8]) -> Result<Vec<u8>, Base16Error> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    if !data.len().is_multiple_of(2) {
        return Err(Base16Error::InvalidLength(data.len()));
    }

    let output_len = data.len() / 2;
    let mut output = vec![0u8; output_len];
    let _ = decode_into(config, &mut output, data)?;
    Ok(output)
}
