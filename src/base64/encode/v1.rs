use super::super::config::Base64Config;

pub fn encode_v1(config: &Base64Config, data: &[u8]) -> Vec<u8> {
    if data.is_empty() {
        return Vec::new();
    }

    let full_chunks = data.len() / 3;
    let remainder = data.len() % 3;
    let output_len = full_chunks * 4
        + match remainder {
            0 => 0,
            1 => 4,
            2 => 4,
            _ => unreachable!(),
        };

    let mut output = Vec::with_capacity(output_len);
    let alphabet = config.alphabet;

    for chunk in data[..full_chunks * 3].chunks_exact(3) {
        let triple = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
        let buf = [
            alphabet[(triple >> 18) as usize & 0x3F],
            alphabet[(triple >> 12) as usize & 0x3F],
            alphabet[(triple >> 6) as usize & 0x3F],
            alphabet[triple as usize & 0x3F],
        ];
        output.extend_from_slice(&buf);
    }

    match remainder {
        1 => {
            let triple = (data[data.len() - 1] as u32) << 16;
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;
            let buf = [alphabet[c0], alphabet[c1], config.padding, config.padding];
            output.extend_from_slice(&buf);
        }
        2 => {
            let triple =
                ((data[data.len() - 2] as u32) << 16) | ((data[data.len() - 1] as u32) << 8);
            let c0 = ((triple >> 18) & 0x3F) as usize;
            let c1 = ((triple >> 12) & 0x3F) as usize;
            let c2 = ((triple >> 6) & 0x3F) as usize;
            let buf = [alphabet[c0], alphabet[c1], alphabet[c2], config.padding];
            output.extend_from_slice(&buf);
        }
        0 => {}
        _ => unreachable!(),
    }

    output
}
