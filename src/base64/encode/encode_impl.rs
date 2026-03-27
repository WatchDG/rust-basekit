use super::super::config::Base64EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base64EncodeOutput;

#[inline]
pub fn encode(config: &Base64EncodeConfig, data: impl AsRef<[u8]>) -> Base64EncodeOutput {
    let data = data.as_ref();
    if data.is_empty() {
        return Base64EncodeOutput { inner: Vec::new() };
    }

    let full_groups = data.len() / 3;
    let remainder = data.len() % 3;
    let output_len = full_groups * 4
        + match (remainder, config.padding.is_some()) {
            (0, _) => 0,
            (1, true) => 4,
            (1, false) => 2,
            (2, true) => 4,
            (2, false) => 3,
            _ => unreachable!(),
        };

    let mut output = Vec::with_capacity(output_len);
    unsafe { output.set_len(output_len) };
    let _ = encode_into(config, &mut output, data).unwrap();
    Base64EncodeOutput { inner: output }
}
