use super::super::config::Base16EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base16EncodeOutput;

pub fn encode(config: &Base16EncodeConfig, data: &[u8]) -> Base16EncodeOutput {
    if data.is_empty() {
        return Base16EncodeOutput { inner: Vec::new() };
    }

    let output_len = data.len() * 2;
    let mut output = vec![0u8; output_len];
    let _ = encode_into(config, &mut output, data).unwrap();
    Base16EncodeOutput { inner: output }
}
