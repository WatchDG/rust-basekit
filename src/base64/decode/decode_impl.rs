use super::super::config::Base64DecodeConfig;
use super::super::error::Base64Error;
use super::decode_into::decode_into;

pub fn decode(config: &Base64DecodeConfig, data: &[u8]) -> Result<Vec<u8>, Base64Error> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut padding_count = 0;
    for &byte in data.iter().rev() {
        if byte == config.padding {
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
        return Ok(Vec::new());
    }

    let output_len = (clean_len * 3) / 4;
    let mut output = vec![0u8; output_len];
    let _ = decode_into(config, &mut output, data)?;
    Ok(output)
}
