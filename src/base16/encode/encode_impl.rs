use super::super::config::Base16EncodeConfig;
use super::encode_into::encode16_into;
use super::output::Base16EncodeOutput;
use crate::utils::init_vec_with;

#[inline]
pub fn encode16(config: &Base16EncodeConfig, data: impl AsRef<[u8]>) -> Base16EncodeOutput {
    let data = data.as_ref();

    let full_groups_count = data.len();
    let _remainder = 0;
    let tail_output_len = 0;
    let output_len = full_groups_count * 2 + tail_output_len;

    let output =
        unsafe { init_vec_with(output_len, |buf| encode16_into(config, buf, data)).unwrap() };

    Base16EncodeOutput { inner: output }
}
