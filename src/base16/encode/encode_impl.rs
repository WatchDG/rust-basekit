use super::super::config::Base16EncodeConfig;
use super::encode_into::encode_into;
use super::output::Base16EncodeOutput;

pub fn encode(config: &Base16EncodeConfig, data: impl AsRef<[u8]>) -> Base16EncodeOutput {
    let data = data.as_ref();
    if data.is_empty() {
        return Base16EncodeOutput { inner: Vec::new() };
    }

    let output_len = data.len() * 2;

    let mut output = Vec::with_capacity(output_len);
    unsafe { output.set_len(output_len) };

    let _ = encode_into(config, &mut output, data).unwrap();
    Base16EncodeOutput { inner: output }
}
