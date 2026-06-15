use super::super::config::Base64EncodeConfig;
use super::encode_into::encode64_into;
use super::output::Base64EncodeOutput;
use crate::utils::init_vec_with;

#[inline]
pub fn encode64(config: &Base64EncodeConfig, data: impl AsRef<[u8]>) -> Base64EncodeOutput {
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

    let output =
        unsafe { init_vec_with(output_len, |buf| encode64_into(config, buf, data)).unwrap() };

    Base64EncodeOutput { inner: output }
}
