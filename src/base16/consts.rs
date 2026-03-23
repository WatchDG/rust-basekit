pub const ALPHABET_BASE16_UPPERCASE: &[u8; 16] = b"0123456789ABCDEF";
pub const ALPHABET_BASE16_LOWERCASE: &[u8; 16] = b"0123456789abcdef";

const fn generate_decode_table(alphabet: &[u8; 16]) -> [i8; 128] {
    let mut table = [-1i8; 128];
    let mut i = 0;
    while i < 16 {
        table[alphabet[i] as usize] = i as i8;
        i += 1;
    }
    table
}

pub const DECODE_TABLE_BASE16_UPPERCASE: &[i8; 128] =
    &generate_decode_table(ALPHABET_BASE16_UPPERCASE);
pub const DECODE_TABLE_BASE16_LOWERCASE: &[i8; 128] =
    &generate_decode_table(ALPHABET_BASE16_LOWERCASE);
