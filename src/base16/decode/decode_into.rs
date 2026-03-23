use super::super::config::Base16DecodeConfig;
use super::super::error::Base16Error;

#[inline(always)]
pub fn decode_into(
    config: &Base16DecodeConfig,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, Base16Error> {
    if src.is_empty() {
        return Ok(0);
    }

    if !src.len().is_multiple_of(2) {
        return Err(Base16Error::InvalidLength(src.len()));
    }

    let output_len = src.len() / 2;

    if dst.len() < output_len {
        return Err(Base16Error::DestinationBufferTooSmall {
            needed: output_len,
            provided: dst.len(),
        });
    }

    let decode_table = config.decode_table;

    unsafe {
        let mut src_offset = 0usize;
        let mut dst_offset = 0usize;

        while src_offset < src.len() {
            let high_nibble = src[src_offset];
            let low_nibble = src[src_offset + 1];

            if high_nibble >= 128 {
                return Err(Base16Error::InvalidCharacter(high_nibble, src_offset));
            }
            if low_nibble >= 128 {
                return Err(Base16Error::InvalidCharacter(low_nibble, src_offset + 1));
            }

            let high_val = decode_table[high_nibble as usize];
            let low_val = decode_table[low_nibble as usize];

            if high_val < 0 {
                return Err(Base16Error::InvalidCharacter(high_nibble, src_offset));
            }
            if low_val < 0 {
                return Err(Base16Error::InvalidCharacter(low_nibble, src_offset + 1));
            }

            let ptr = dst.as_mut_ptr().add(dst_offset);
            ptr.write(((high_val as u8) << 4) | (low_val as u8));

            src_offset += 2;
            dst_offset += 1;
        }

        Ok(output_len)
    }
}
