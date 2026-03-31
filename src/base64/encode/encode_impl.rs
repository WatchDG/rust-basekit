use super::super::config::Base64EncodeConfig;
use super::encode_into_slice;
use super::output::Base64EncodeOutput;

#[inline]
pub fn encode(config: &Base64EncodeConfig, data: impl AsRef<[u8]>) -> Base64EncodeOutput {
    let data = data.as_ref();

    if data.is_empty() {
        return Base64EncodeOutput { inner: Vec::new() };
    }

    let full_groups_count = data.len() / 3;
    let remainder = data.len() % 3;
    let output_len = full_groups_count * 4
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

    let full_groups_src = if full_groups_count > 0 {
        Some(&data[..full_groups_count * 3])
    } else {
        None
    };
    let tail_src = if remainder > 0 {
        Some(&data[full_groups_count * 3..])
    } else {
        None
    };

    let _ = encode_into_slice(config, &mut output, full_groups_src, tail_src).unwrap();

    Base64EncodeOutput { inner: output }
}
