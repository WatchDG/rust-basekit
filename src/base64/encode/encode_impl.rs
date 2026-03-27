use super::super::config::Base64EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base64EncodeOutput;

#[inline]
pub fn encode(config: &Base64EncodeConfig, data: impl AsRef<[u8]>) -> Base64EncodeOutput {
    let data = data.as_ref();
    if data.is_empty() {
        return Base64EncodeOutput { inner: Vec::new() };
    }

    let full_chunks = data.len() / 3;
    let remainder = data.len() % 3;
    let output_len = if config.padding.is_some() {
        full_chunks * 4
            + match remainder {
                0 => 0,
                1 => 4,
                2 => 4,
                _ => unreachable!(),
            }
    } else {
        full_chunks * 4
            + match remainder {
                0 => 0,
                1 => 2,
                2 => 3,
                _ => unreachable!(),
            }
    };

    let mut output = Vec::with_capacity(output_len);
    unsafe { output.set_len(output_len) };
    let _ = encode_into(config, &mut output, data).unwrap();
    Base64EncodeOutput { inner: output }
}
