use super::super::config::Base64EncodeConfig;
use super::encode_into::encode_into;

pub fn encode(config: &Base64EncodeConfig, data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let full_chunks = data.len() / 3;
    let remainder = data.len() % 3;
    let output_len = full_chunks * 4
        + match remainder {
            0 => 0,
            1 => 4,
            2 => 4,
            _ => unreachable!(),
        };

    let mut output = vec![0u8; output_len];
    let _ = encode_into(config, &mut output, data).unwrap();
    output
}
