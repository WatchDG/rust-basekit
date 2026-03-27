use super::super::config::Base32EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base32EncodeOutput;

pub fn encode(config: &Base32EncodeConfig, data: impl AsRef<[u8]>) -> Base32EncodeOutput {
    let data = data.as_ref();
    if data.is_empty() {
        return Base32EncodeOutput { inner: Vec::new() };
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
    Base32EncodeOutput { inner: output }
}
