use super::super::config::Base32EncodeConfig;
use super::encode_into::encode32_into;
use super::output::Base32EncodeOutput;
use crate::utils::init_vec_with;

#[inline]
pub fn encode32(config: &Base32EncodeConfig, data: impl AsRef<[u8]>) -> Base32EncodeOutput {
    let data = data.as_ref();

    if data.is_empty() {
        return Base32EncodeOutput { inner: Vec::new() };
    }

    let full_groups_count = data.len() / 5;
    let remainder = data.len() % 5;
    let tail_output_len = match (remainder, config.padding.is_some()) {
        (0, _) => 0,
        (1, true) => 8,
        (1, false) => 2,
        (2, true) => 8,
        (2, false) => 4,
        (3, true) => 8,
        (3, false) => 5,
        (4, true) => 8,
        (4, false) => 7,
        _ => unreachable!(),
    };
    let output_len = full_groups_count * 8 + tail_output_len;

    let output =
        unsafe { init_vec_with(output_len, |buf| encode32_into(config, buf, data)).unwrap() };

    Base32EncodeOutput { inner: output }
}
