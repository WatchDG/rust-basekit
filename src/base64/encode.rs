use super::config::Base64Config;

pub fn encode_v1(config: &Base64Config, data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let output_len = data.len().div_ceil(3) * 4;
    let mut output = Vec::with_capacity(output_len);

    let chunks = data.chunks(3);

    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        let c0 = ((triple >> 18) & 0x3F) as usize;
        let c1 = ((triple >> 12) & 0x3F) as usize;
        let c2 = ((triple >> 6) & 0x3F) as usize;
        let c3 = (triple & 0x3F) as usize;

        output.push(config.alphabet[c0]);
        output.push(config.alphabet[c1]);

        if chunk.len() == 3 {
            output.push(config.alphabet[c2]);
            output.push(config.alphabet[c3]);
        } else if chunk.len() == 2 {
            output.push(config.alphabet[c2]);
            output.push(config.padding);
        } else {
            output.push(config.padding);
            output.push(config.padding);
        }
    }

    output
}
