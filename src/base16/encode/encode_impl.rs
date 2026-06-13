use super::super::config::Base16EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base16EncodeOutput;
use crate::utils::init_vec_with;

#[inline]
pub fn encode(config: &Base16EncodeConfig, data: impl AsRef<[u8]>) -> Base16EncodeOutput {
    let data = data.as_ref();

    if data.is_empty() {
        return Base16EncodeOutput { inner: Vec::new() };
    }

    let output_len = data.len() * 2;

    let output =
        unsafe { init_vec_with(output_len, |buf| encode_into(config, buf, data)).unwrap() };

    Base16EncodeOutput { inner: output }
}
