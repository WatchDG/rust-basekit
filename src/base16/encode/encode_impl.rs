use super::super::config::Base16EncodeConfig;
use super::encode_into::encode_into;

pub fn encode(config: &Base16EncodeConfig, data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let output_len = data.len() * 2;
    let mut output = vec![0u8; output_len];
    let _ = encode_into(config, &mut output, data).unwrap();
    output
}
