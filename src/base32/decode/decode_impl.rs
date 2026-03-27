use super::super::config::Base32DecodeConfig;
use super::super::error::Base32Error;
use super::decode_into::decode_into;
use super::output::Base32DecodeOutput;

pub fn decode(
    config: &Base32DecodeConfig,
    data: impl AsRef<[u8]>,
) -> Result<Base32DecodeOutput, Base32Error> {
    let data = data.as_ref();
    if data.is_empty() {
        return Ok(Base32DecodeOutput { inner: Vec::new() });
    }

    let mut padding_count = 0;
    if let Some(padding) = config.padding {
        for &byte in data.iter().rev() {
            if byte == padding {
                padding_count += 1;
            } else {
                break;
            }
        }
    }

    if padding_count > 6 {
        return Err(Base32Error::InvalidPadding);
    }

    let clean_len = data.len() - padding_count;

    if clean_len == 0 {
        return Ok(Base32DecodeOutput { inner: Vec::new() });
    }

    let output_len = (clean_len * 5) / 8;
    let mut inner = vec![0u8; output_len];
    let _ = decode_into(config, &mut inner, data)?;
    Ok(Base32DecodeOutput { inner })
}
