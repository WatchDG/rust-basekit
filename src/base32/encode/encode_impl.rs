use super::super::config::Base32EncodeConfig;
use super::encode_into::encode_into;

pub fn encode(config: &Base32EncodeConfig, data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let full_chunks = data.len() / 5;
    let remainder = data.len() % 5;
    let output_len = full_chunks * 8
        + match remainder {
            0 => 0,
            1 => 8,
            2 => 8,
            3 => 8,
            4 => 8,
            _ => unreachable!(),
        };

    let mut output = vec![config.padding; output_len];
    let _ = encode_into(config, &mut output, data).unwrap();
    output
}
