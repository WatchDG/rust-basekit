use super::super::config::Base64EncodeConfig;

pub fn encode(config: &Base64EncodeConfig, data: &[u8]) -> Vec<u8> {
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

    let mut output: Vec<u8> = Vec::with_capacity(output_len);
    let alphabet_ptr = config.alphabet.as_ptr();
    let padding = config.padding;

    unsafe {
        let mut offset = 0;

        for chunk in data[..full_chunks * 3].chunks_exact(3) {
            let triple = ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32);
            let ptr = output.as_mut_ptr().add(offset);

            ptr.write(alphabet_ptr.offset((triple >> 18 & 0x3F) as isize).read());
            ptr.offset(1)
                .write(alphabet_ptr.offset((triple >> 12 & 0x3F) as isize).read());
            ptr.offset(2)
                .write(alphabet_ptr.offset((triple >> 6 & 0x3F) as isize).read());
            ptr.offset(3)
                .write(alphabet_ptr.offset((triple & 0x3F) as isize).read());

            offset += 4;
        }

        match remainder {
            1 => {
                let triple = (data[data.len() - 1] as u32) << 16;
                let c0 = ((triple >> 18) & 0x3F) as isize;
                let c1 = ((triple >> 12) & 0x3F) as isize;
                let ptr = output.as_mut_ptr().add(offset);

                ptr.write(alphabet_ptr.offset(c0).read());
                ptr.offset(1).write(alphabet_ptr.offset(c1).read());
                ptr.offset(2).write(padding);
                ptr.offset(3).write(padding);
            }
            2 => {
                let triple =
                    ((data[data.len() - 2] as u32) << 16) | ((data[data.len() - 1] as u32) << 8);
                let c0 = ((triple >> 18) & 0x3F) as isize;
                let c1 = ((triple >> 12) & 0x3F) as isize;
                let c2 = ((triple >> 6) & 0x3F) as isize;
                let ptr = output.as_mut_ptr().add(offset);

                ptr.write(alphabet_ptr.offset(c0).read());
                ptr.offset(1).write(alphabet_ptr.offset(c1).read());
                ptr.offset(2).write(alphabet_ptr.offset(c2).read());
                ptr.offset(3).write(padding);
            }
            0 => {}
            _ => unreachable!(),
        }

        output.set_len(output_len);
    }

    output
}
